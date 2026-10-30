//! Hosted Boxwren API. Software fault isolation is not a VM — see docs/HOSTED.md.

mod db;
mod policy;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use boxwren::{
    DirGrant, Entry, EnvGrant, Grants, KillHandle, Limits, NetGrant, Report, RunRequest, Status,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::db::{Account, ApiKeyRow, Db, RunRow};
use crate::policy::preflight_keywarden;

#[derive(Parser, Debug)]
#[command(name = "boxwren-server")]
struct Args {
    #[arg(long, env = "BOXWREN_LISTEN", default_value = "127.0.0.1:8787")]
    listen: SocketAddr,
    #[arg(long, env = "BOXWREN_DB", default_value = "boxwren-server.sqlite")]
    db: PathBuf,
    /// Optional Keywarden admission URL. POST JSON `{ "action": "boxwren.run", ... }`.
    #[arg(long, env = "KEYWARDEN_URL")]
    keywarden_url: Option<String>,
}

struct AppState {
    db: Db,
    engine: boxwren::Engine,
    inflight: Mutex<HashMap<String, KillHandle>>,
    keywarden_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "boxwren_server=info,tower_http=info".into()),
        )
        .init();
    let args = Args::parse();
    let db = Db::open(&args.db)?;
    let engine = boxwren::Engine::new()?;
    let state = Arc::new(AppState {
        db,
        engine,
        inflight: Mutex::new(HashMap::new()),
        keywarden_url: args.keywarden_url,
    });

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/v1/accounts", post(create_account))
        .route("/v1/me", get(me))
        .route("/v1/keys", get(list_keys).post(create_key))
        .route("/v1/runs", get(list_runs).post(create_run))
        .route("/v1/runs/{id}", get(get_run))
        .route("/v1/runs/{id}/kill", post(kill_run))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tracing::info!(%args.listen, db = %args.db.display(), "listening");
    let listener = tokio::net::TcpListener::bind(args.listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CreateAccount {
    email: String,
}

#[derive(Serialize)]
struct CreateAccountResponse {
    id: String,
    email: String,
    api_key: String,
}

async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateAccount>,
) -> Result<Json<CreateAccountResponse>, ApiError> {
    let (account, api_key) = state.db.create_account(&body.email)?;
    Ok(Json(CreateAccountResponse {
        id: account.id,
        email: account.email,
        api_key,
    }))
}

async fn me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Account>, ApiError> {
    let account = require_account(&state, &headers)?;
    Ok(Json(account))
}

#[derive(Deserialize)]
struct CreateKey {
    name: String,
}

#[derive(Serialize)]
struct CreateKeyResponse {
    id: String,
    name: String,
    api_key: String,
}

async fn create_key(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateKey>,
) -> Result<Json<CreateKeyResponse>, ApiError> {
    let account = require_account(&state, &headers)?;
    let (row, api_key) = state.db.create_key(&account.id, &body.name)?;
    Ok(Json(CreateKeyResponse {
        id: row.id,
        name: row.name,
        api_key,
    }))
}

#[derive(Serialize)]
struct KeysResponse {
    keys: Vec<ApiKeyRow>,
}

async fn list_keys(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<KeysResponse>, ApiError> {
    let account = require_account(&state, &headers)?;
    Ok(Json(KeysResponse {
        keys: state.db.list_keys(&account.id)?,
    }))
}

#[derive(Deserialize)]
struct CreateRun {
    /// Base64-encoded wasm bytes. Prefer this for binary modules.
    #[serde(default)]
    wasm_base64: Option<String>,
    /// WAT or raw wasm text for small demos.
    #[serde(default)]
    wasm_text: Option<String>,
    #[serde(default = "default_entry")]
    entry: String,
    #[serde(default)]
    args: Vec<String>,
    fuel: Option<u64>,
    memory_bytes: Option<usize>,
    wall_ms: Option<u64>,
    #[serde(default)]
    dirs: Vec<String>,
    #[serde(default)]
    net: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
}

fn default_entry() -> String {
    "_start".into()
}

#[derive(Serialize)]
struct CreateRunResponse {
    id: String,
    report: Report,
}

async fn create_run(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateRun>,
) -> Result<Json<CreateRunResponse>, ApiError> {
    let account = require_account(&state, &headers)?;
    if let Some(url) = &state.keywarden_url {
        preflight_keywarden(url, &account.id, &body.entry)
            .await
            .map_err(|err| ApiError::forbidden(err))?;
    }

    let wasm = decode_wasm(&body)?;
    let defaults = Limits::default();
    let limits = Limits {
        fuel: body.fuel.unwrap_or(defaults.fuel),
        memory_bytes: body.memory_bytes.unwrap_or(defaults.memory_bytes),
        wall_time: body
            .wall_ms
            .map(Duration::from_millis)
            .unwrap_or(defaults.wall_time),
        ..defaults
    };
    let mut grants = Grants::default();
    for spec in &body.dirs {
        grants
            .dirs
            .push(DirGrant::parse(spec).map_err(|e| ApiError::bad(e.to_string()))?);
    }
    for spec in &body.net {
        grants
            .net
            .push(NetGrant::parse(spec).map_err(|e| ApiError::bad(e.to_string()))?);
    }
    for spec in &body.env {
        grants
            .env
            .push(EnvGrant::parse(spec).map_err(|e| ApiError::bad(e.to_string()))?);
    }
    let entry = if body.entry == "_start" {
        let mut args = body.args;
        if args.is_empty() {
            args.push("guest".into());
        }
        Entry::Command { args }
    } else {
        Entry::Export(body.entry.clone())
    };

    let id = Uuid::new_v4().to_string();
    let kill = state.engine.kill_handle();
    {
        let mut guard = state.inflight.lock().await;
        guard.insert(id.clone(), kill.clone());
    }

    let engine = state.engine.clone();
    let request = RunRequest {
        wasm,
        entry,
        limits,
        grants,
        args: Vec::new(),
    };
    let report = tokio::task::spawn_blocking(move || engine.execute_with(request, &kill))
        .await
        .map_err(|err| ApiError::internal(err.to_string()))?
        .map_err(|err| ApiError::bad(err.to_string()))?;

    {
        let mut guard = state.inflight.lock().await;
        guard.remove(&id);
    }

    let status = status_kind(&report.status);
    state.db.insert_run(
        &id,
        &account.id,
        status,
        report.duration_ms,
        report.fuel_used,
        &serde_json::to_string(&report).unwrap_or_else(|_| "{}".into()),
    )?;

    Ok(Json(CreateRunResponse { id, report }))
}

#[derive(Serialize)]
struct RunsResponse {
    runs: Vec<RunRow>,
}

async fn list_runs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<RunsResponse>, ApiError> {
    let account = require_account(&state, &headers)?;
    Ok(Json(RunsResponse {
        runs: state.db.list_runs(&account.id)?,
    }))
}

#[derive(Serialize)]
struct GetRunResponse {
    id: String,
    status: String,
    report: serde_json::Value,
}

async fn get_run(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<GetRunResponse>, ApiError> {
    let account = require_account(&state, &headers)?;
    let row = state
        .db
        .get_run(&account.id, &id)?
        .ok_or_else(|| ApiError::not_found("run not found"))?;
    let report: serde_json::Value =
        serde_json::from_str(&row.report_json).unwrap_or(serde_json::json!({}));
    Ok(Json(GetRunResponse {
        id: row.id,
        status: row.status,
        report,
    }))
}

async fn kill_run(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let _account = require_account(&state, &headers)?;
    let guard = state.inflight.lock().await;
    if let Some(kill) = guard.get(&id) {
        kill.kill();
        Ok(StatusCode::ACCEPTED)
    } else {
        Err(ApiError::not_found("run not in flight"))
    }
}

fn decode_wasm(body: &CreateRun) -> Result<Vec<u8>, ApiError> {
    if let Some(b64) = &body.wasm_base64 {
        use base64::Engine as _;
        return base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|err| ApiError::bad(format!("wasm_base64: {err}")));
    }
    if let Some(text) = &body.wasm_text {
        return Ok(text.as_bytes().to_vec());
    }
    Err(ApiError::bad(
        "provide wasm_base64 or wasm_text".into(),
    ))
}

fn status_kind(status: &Status) -> &'static str {
    match status {
        Status::Success => "success",
        Status::FuelExhausted => "fuel_exhausted",
        Status::MemoryLimited => "memory_limited",
        Status::TimedOut => "timed_out",
        Status::Killed => "killed",
        Status::Trapped { .. } => "trapped",
        Status::Rejected { .. } => "rejected",
    }
}

fn require_account(state: &AppState, headers: &HeaderMap) -> Result<Account, ApiError> {
    let raw = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::unauthorized("missing Authorization"))?;
    let token = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
        .unwrap_or(raw)
        .trim();
    state
        .db
        .account_for_key(token)?
        .ok_or_else(|| ApiError::unauthorized("invalid API key"))
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn bad(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message,
        }
    }
    fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }
    fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: message.into(),
        }
    }
    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        ApiError::internal(message)
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}
