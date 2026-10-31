use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use boxwren::{DirGrant, Entry, EnvGrant, Grants, Limits, NetGrant, Report, RunRequest, Status};
use clap::{Parser, Subcommand, ValueEnum};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(args) => match run(args) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("boxwren: {err}");
                ExitCode::from(2)
            }
        },
        #[cfg(feature = "interpreters")]
        Command::RunPy(args) => match run_script(args, ScriptKind::Python) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("boxwren: {err}");
                ExitCode::from(2)
            }
        },
        #[cfg(feature = "interpreters")]
        Command::RunJs(args) => match run_script(args, ScriptKind::JavaScript) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("boxwren: {err}");
                ExitCode::from(2)
            }
        },
        Command::Demo { report } => match demo(report) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("boxwren: {err}");
                ExitCode::from(2)
            }
        },
    }
}

#[derive(Parser)]
#[command(
    name = "boxwren",
    version,
    about = "Run an untrusted WASI module with deny-by-default limits."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run one module and print an execution report.
    Run(RunArgs),
    /// Run a Python source file inside the bundled MicroPython WASI guest.
    #[cfg(feature = "interpreters")]
    RunPy(ScriptArgs),
    /// Run a JavaScript source file inside the bundled QuickJS WASI guest.
    #[cfg(feature = "interpreters")]
    RunJs(ScriptArgs),
    /// Run the blocked secret, network, and memory guests.
    Demo {
        /// `text` or `json`.
        #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
        report: ReportFormat,
    },
}

#[cfg(feature = "interpreters")]
#[derive(Parser)]
struct ScriptArgs {
    /// Path to a `.py` or `.js` source file.
    script: PathBuf,
    /// `text` or `json`.
    #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
    report: ReportFormat,
    #[arg(long)]
    fuel: Option<u64>,
    #[arg(long, value_name = "BYTES")]
    memory: Option<String>,
    #[arg(long, value_name = "DURATION")]
    timeout: Option<String>,
    #[arg(long = "allow-dir", value_name = "HOST:GUEST[:ro|rw]")]
    allow_dir: Vec<String>,
    #[arg(long = "allow-net", value_name = "HOST[:PORT]")]
    allow_net: Vec<String>,
    #[arg(long = "allow-env", value_name = "NAME[=VALUE]")]
    allow_env: Vec<String>,
}

#[cfg(feature = "interpreters")]
enum ScriptKind {
    Python,
    JavaScript,
}

#[derive(Parser)]
struct RunArgs {
    /// Path to a `.wasm` or `.wat` module.
    module: PathBuf,
    /// `text` or `json`.
    #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
    report: ReportFormat,
    /// Instruction budget. This is not milliseconds.
    #[arg(long)]
    fuel: Option<u64>,
    /// Maximum linear memory. Examples: `32MiB`, `65536`.
    #[arg(long, value_name = "BYTES")]
    memory: Option<String>,
    /// Wall-clock deadline. Examples: `2s`, `500ms`.
    #[arg(long, value_name = "DURATION")]
    timeout: Option<String>,
    /// `host:guest`, `host:guest:ro`, or `host:guest:rw`. Default mode is read-only.
    #[arg(long = "allow-dir", value_name = "HOST:GUEST[:ro|rw]")]
    allow_dir: Vec<String>,
    /// `ip`, `ip:port`, `hostname`, or `hostname:port`. A bare target allows every port.
    #[arg(long = "allow-net", value_name = "HOST[:PORT]")]
    allow_net: Vec<String>,
    /// `NAME` copies the host value. `NAME=VALUE` sets it explicitly.
    #[arg(long = "allow-env", value_name = "NAME[=VALUE]")]
    allow_env: Vec<String>,
    /// Arguments after the module name, passed as the guest argv.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Clone, Copy, ValueEnum)]
enum ReportFormat {
    Text,
    Json,
}

#[cfg(feature = "interpreters")]
fn run_script(args: ScriptArgs, kind: ScriptKind) -> Result<ExitCode, String> {
    let limits = Limits {
        fuel: args.fuel.unwrap_or(Limits::default().fuel),
        memory_bytes: match &args.memory {
            Some(memory) => parse_bytes(memory)?,
            None => Limits::default().memory_bytes,
        },
        wall_time: match &args.timeout {
            Some(timeout) => parse_duration(timeout)?,
            None => Limits::default().wall_time,
        },
        ..Limits::default()
    };
    let mut grants = Grants::default();
    for spec in &args.allow_dir {
        grants
            .dirs
            .push(DirGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    for spec in &args.allow_net {
        grants
            .net
            .push(NetGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    for spec in &args.allow_env {
        grants
            .env
            .push(EnvGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    let engine = boxwren::Engine::new().map_err(|err| err.to_string())?;
    let kill = engine.kill_handle();
    let interrupt = kill.clone();
    ctrlc::set_handler(move || interrupt.kill()).map_err(|err| err.to_string())?;
    let report = match kind {
        ScriptKind::Python => {
            boxwren::interpreters::execute_python_file(&engine, &args.script, limits, grants)
        }
        ScriptKind::JavaScript => {
            boxwren::interpreters::execute_javascript_file(&engine, &args.script, limits, grants)
        }
    }
    .map_err(|err| err.to_string())?;
    print_report(&report, args.report);
    Ok(exit_code(&report))
}

fn run(args: RunArgs) -> Result<ExitCode, String> {
    let wasm = std::fs::read(&args.module)
        .map_err(|err| format!("could not read {}: {err}", args.module.display()))?;
    let limits = limits_from(&args)?;
    let grants = grants_from(&args)?;
    let name = args
        .module
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "guest".into());
    let mut argv = vec![name];
    argv.extend(args.args);
    let engine = boxwren::Engine::new().map_err(|err| err.to_string())?;
    let kill = engine.kill_handle();
    let interrupt = kill.clone();
    ctrlc::set_handler(move || interrupt.kill()).map_err(|err| err.to_string())?;
    if std::env::var_os("BOXWREN_READY").is_some() {
        eprintln!("boxwren: ready");
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }
    let report = engine
        .execute_with(
            RunRequest {
                wasm,
                entry: Entry::Command { args: argv },
                limits,
                grants,
                args: Vec::new(),
            },
            &kill,
        )
        .map_err(|err| err.to_string())?;
    print_report(&report, args.report);
    Ok(exit_code(&report))
}

fn limits_from(args: &RunArgs) -> Result<Limits, String> {
    let defaults = Limits::default();
    Ok(Limits {
        fuel: args.fuel.unwrap_or(defaults.fuel),
        memory_bytes: match &args.memory {
            Some(memory) => parse_bytes(memory)?,
            None => defaults.memory_bytes,
        },
        wall_time: match &args.timeout {
            Some(timeout) => parse_duration(timeout)?,
            None => defaults.wall_time,
        },
        ..defaults
    })
}

fn grants_from(args: &RunArgs) -> Result<Grants, String> {
    let mut grants = Grants::default();
    for spec in &args.allow_dir {
        grants
            .dirs
            .push(DirGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    for spec in &args.allow_net {
        grants
            .net
            .push(NetGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    for spec in &args.allow_env {
        grants
            .env
            .push(EnvGrant::parse(spec).map_err(|err| err.to_string())?);
    }
    Ok(grants)
}

fn print_report(report: &Report, format: ReportFormat) {
    match format {
        ReportFormat::Json => println!("{}", report.to_json()),
        ReportFormat::Text => print!("{}", report.to_text()),
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

fn exit_code(report: &Report) -> ExitCode {
    if matches!(report.status, Status::Success) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn parse_bytes(text: &str) -> Result<usize, String> {
    let (number, shift) = if let Some(rest) =
        strip_suffix_ci(text, "gib").or_else(|| strip_suffix_ci(text, "gb"))
    {
        (rest, 30u32)
    } else if let Some(rest) = strip_suffix_ci(text, "mib").or_else(|| strip_suffix_ci(text, "mb"))
    {
        (rest, 20)
    } else if let Some(rest) = strip_suffix_ci(text, "kib").or_else(|| strip_suffix_ci(text, "kb"))
    {
        (rest, 10)
    } else {
        (text, 0)
    };
    let number = number.trim();
    let value: u64 = number
        .parse()
        .map_err(|_| format!("memory `{text}` is not a byte count"))?;
    let shifted = value
        .checked_shl(shift)
        .ok_or_else(|| format!("memory `{text}` is too large"))?;
    usize::try_from(shifted).map_err(|_| format!("memory `{text}` is too large"))
}

fn strip_suffix_ci<'a>(text: &'a str, suffix: &str) -> Option<&'a str> {
    if text.len() >= suffix.len() && text[text.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
    {
        Some(&text[..text.len() - suffix.len()])
    } else {
        None
    }
}

fn parse_duration(text: &str) -> Result<Duration, String> {
    if let Some(rest) = text.strip_suffix("ms") {
        let ms: u64 = rest
            .trim()
            .parse()
            .map_err(|_| format!("timeout `{text}` is not a duration"))?;
        return Ok(Duration::from_millis(ms));
    }
    if let Some(rest) = text.strip_suffix('s') {
        let secs: u64 = rest
            .trim()
            .parse()
            .map_err(|_| format!("timeout `{text}` is not a duration"))?;
        return Ok(Duration::from_secs(secs));
    }
    let ms: u64 = text
        .parse()
        .map_err(|_| format!("timeout `{text}` must look like 500ms or 2s"))?;
    Ok(Duration::from_millis(ms))
}

const SECRET_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/secret.wasm"));
const NET_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/net.wasm"));
const MEMORY_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/memory.wasm"));

type DemoCase = (&'static str, &'static [u8], fn(&Report) -> bool);

fn demo(format: ReportFormat) -> Result<ExitCode, String> {
    let engine = boxwren::Engine::new().map_err(|err| err.to_string())?;
    let limits = Limits {
        wall_time: Duration::from_secs(5),
        ..Limits::default()
    };
    let cases: [DemoCase; 3] = [
        ("secret", SECRET_WASM, secret_blocked),
        ("network", NET_WASM, network_blocked),
        ("memory", MEMORY_WASM, memory_blocked),
    ];
    let mut failed = false;
    for (name, wasm, blocked) in cases {
        let report = engine
            .execute(RunRequest {
                wasm: wasm.to_vec(),
                entry: Entry::Command {
                    args: vec![name.into()],
                },
                limits,
                grants: Grants::default(),
                args: Vec::new(),
            })
            .map_err(|err| err.to_string())?;
        let ok = blocked(&report);
        if !ok {
            failed = true;
        }
        println!("# {name}: {}", if ok { "blocked" } else { "ALLOWED" });
        print_report(&report, format);
    }
    if failed {
        eprintln!("boxwren: demo failed; an abuse case was allowed");
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn secret_blocked(report: &Report) -> bool {
    !report.stdout.contains("SECRET")
        && report.events.iter().any(|event| {
            event.kind == boxwren::EventKind::Filesystem
                && event.target.contains("passwd")
                && event.decision == boxwren::Decision::Denied
        })
}

fn network_blocked(report: &Report) -> bool {
    !report.stdout.contains("CONNECTED")
        && report.events.iter().any(|event| {
            event.kind == boxwren::EventKind::Network
                && event.target == "127.0.0.1:1"
                && event.decision == boxwren::Decision::Denied
        })
}

fn memory_blocked(report: &Report) -> bool {
    matches!(report.status, Status::MemoryLimited) && !report.stdout.contains("allocated 64")
}
