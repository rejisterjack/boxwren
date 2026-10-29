use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: u32 = 1;

/// Outcome of one run. Stable as JSON schema version 1.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: u32,
    pub status: Status,
    /// Process exit code when the guest called `proc_exit`.
    pub exit_code: Option<i32>,
    /// `i32` or `i64` result of an exported function, when it has one.
    pub return_value: Option<i64>,
    pub duration_ms: u64,
    /// Instructions consumed. Not milliseconds.
    pub fuel_used: u64,
    pub peak_memory: u64,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub events: Vec<Event>,
    /// Embedder footguns, such as a grant of the filesystem root.
    pub warnings: Vec<String>,
}

impl Report {
    pub fn success(&self) -> bool {
        matches!(self.status, Status::Success)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Status {
    Success,
    FuelExhausted,
    MemoryLimited,
    TimedOut,
    Killed,
    Trapped { message: String },
    Rejected { message: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub target: String,
    pub decision: Decision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Filesystem,
    Network,
    Env,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Allowed,
    Denied,
}

impl Report {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("status: {}\n", self.status));
        if let Some(code) = self.exit_code {
            out.push_str(&format!("exit_code: {code}\n"));
        }
        if let Some(value) = self.return_value {
            out.push_str(&format!("return_value: {value}\n"));
        }
        out.push_str(&format!("duration_ms: {}\n", self.duration_ms));
        out.push_str(&format!("fuel_used: {}\n", self.fuel_used));
        out.push_str(&format!("peak_memory: {}\n", self.peak_memory));
        if !self.stdout.is_empty() {
            out.push_str("stdout:\n");
            out.push_str(&self.stdout);
            if !self.stdout.ends_with('\n') {
                out.push('\n');
            }
            if self.stdout_truncated {
                out.push_str("stdout_truncated: true\n");
            }
        }
        if !self.stderr.is_empty() {
            out.push_str("stderr:\n");
            out.push_str(&self.stderr);
            if !self.stderr.ends_with('\n') {
                out.push('\n');
            }
            if self.stderr_truncated {
                out.push_str("stderr_truncated: true\n");
            }
        }
        if !self.events.is_empty() {
            out.push_str("events:\n");
            for event in &self.events {
                out.push_str(&format!(
                    "- {} {} {}\n",
                    event.kind, event.target, event.decision
                ));
            }
        }
        if !self.warnings.is_empty() {
            out.push_str("warnings:\n");
            for warning in &self.warnings {
                out.push_str(&format!("- {warning}\n"));
            }
        }
        out
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::FuelExhausted => write!(f, "fuel_exhausted"),
            Self::MemoryLimited => write!(f, "memory_limited"),
            Self::TimedOut => write!(f, "timed_out"),
            Self::Killed => write!(f, "killed"),
            Self::Trapped { message } => write!(f, "trapped: {message}"),
            Self::Rejected { message } => write!(f, "rejected: {message}"),
        }
    }
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filesystem => write!(f, "filesystem"),
            Self::Network => write!(f, "network"),
            Self::Env => write!(f, "env"),
        }
    }
}

impl std::fmt::Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allowed => write!(f, "allowed"),
            Self::Denied => write!(f, "denied"),
        }
    }
}
