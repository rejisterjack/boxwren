//! Hosted Boxwren API. Software fault isolation is not a VM — see docs/HOSTED.md.

mod db;
mod policy;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use boxwren::{
    DirGrant, Entry, EnvGrant, Grants, KillHandle, Limits, NetGrant, Report, RunRequest, Status,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::db::{Account, ApiKeyRow, Db, RunRow};
use crate::policy::preflight_keywarden;

/// Default browser origins allowed when `BOXWREN_CORS_ORIGINS` is unset.
const DEFAULT_CORS_ORIGINS: &[&str] = &[
    "http://127.0.0.1:3000",
    "http://localhost:3000",
    "https://boxwren.rejisterjack.com",
];

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
    /// Comma-separated CORS allowlist. Defaults to localhost:3000 and the public site.
    #[arg(long, env = "BOXWREN_CORS_ORIGINS")]
    cors_origins: Option<String>,
    /// Max `/v1` requests per client key per window.
    #[arg(long, env = "BOXWREN_RATE_LIMIT", default_value_t = 60)]
    rate_limit: u32,
    /// Rate-limit window in seconds.
    #[arg(long, env = "BOXWREN_RATE_WINDOW_SECS", default_value_t = 60)]
    rate_window_secs: u64,
}

struct RateLimiter {
    limit: u32,
    window: Duration,
    hits: Mutex<HashMap<String, (Instant, u32)>>,
}

impl RateLimiter {
    fn new(limit: u32, window: Duration) -> Self {
        Self {
            limit: limit.max(1),
            window,
            hits: Mutex::new(HashMap::new()),
        }
    }

    async fn check(&self, key: &str) -> bool {
        let mut guard = self.hits.lock().await;
        let now = Instant::now();
        let entry = guard.entry(key.to_string()).or_insert((now, 0));
        if now.duration_since(entry.0) >= self.window {
            *entry = (now, 0);
        }
        if entry.1 >= self.limit {
            return false;
        }
        entry.1 += 1;
        true
    }
}

struct AppState {
    db: Db,
    engine: boxwren::Engine,
    inflight: Mutex<HashMap<String, KillHandle>>,
    keywarden_url: Option<String>,
    limiter: RateLimiter,
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
    // Railway health-checks $PORT. BOXWREN_LISTEN remains the local default.
    let listen = listen_addr(args.listen, std::env::var("PORT").ok().as_deref());
    let db = Db::open(&args.db)?;
    let engine = boxwren::Engine::new()?;
    let state = Arc::new(AppState {
        db,
        engine,
        inflight: Mutex::new(HashMap::new()),
        keywarden_url: args.keywarden_url,
        limiter: RateLimiter::new(
            args.rate_limit,
            Duration::from_secs(args.rate_window_secs.max(1)),
        ),
    });

    let origins = parse_cors_origins(args.cors_origins.as_deref());
    let app = build_app(state, &origins);

    tracing::info!(%listen, db = %args.db.display(), "listening");
    let listener = tokio::net::TcpListener::bind(listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// `PORT` wins when it is a non-zero port, so a platform health check and the
/// process bind the same socket. Anything else keeps the configured address.
fn listen_addr(configured: SocketAddr, port: Option<&str>) -> SocketAddr {
    let Some(raw) = port.map(str::trim).filter(|value| !value.is_empty()) else {
        return configured;
    };
    match raw.parse::<u16>() {
        Ok(port) if port != 0 => SocketAddr::from(([0, 0, 0, 0], port)),
        _ => configured,
    }
}

fn parse_cors_origins(raw: Option<&str>) -> Vec<String> {
    match raw {
        Some(s) if !s.trim().is_empty() => s
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
        _ => DEFAULT_CORS_ORIGINS
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
    }
}

fn cors_layer(origins: &[String]) -> CorsLayer {
    let values: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(values))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            HeaderName::from_static("x-requested-with"),
        ])
}

fn build_app(state: Arc<AppState>, cors_origins: &[String]) -> Router {
    let v1 = Router::new()
        .route("/v1/accounts", post(create_account))
        .route("/v1/me", get(me))
        .route("/v1/keys", get(list_keys).post(create_key))
        .route("/v1/runs", get(list_runs).post(create_run))
        .route("/v1/runs/{id}", get(get_run))
        .route("/v1/runs/{id}/kill", post(kill_run))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ));

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .merge(v1)
        .layer(cors_layer(cors_origins))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let key = rate_limit_key(request.headers());
    if !state.limiter.check(&key).await {
        return Err(ApiError {
            status: StatusCode::TOO_MANY_REQUESTS,
            message: "rate limit exceeded".into(),
        });
    }
    Ok(next.run(request).await)
}

fn rate_limit_key(headers: &HeaderMap) -> String {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| format!("auth:{}", &s[..s.len().min(48)]))
        .unwrap_or_else(|| "anon".into())
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
    /// When false, return the run id immediately and finish in the background
    /// so `POST /v1/runs/{id}/kill` can fire while the guest is live.
    #[serde(default = "default_wait")]
    wait: bool,
}

fn default_entry() -> String {
    "_start".into()
}

fn default_wait() -> bool {
    true
}

#[derive(Serialize)]
struct CreateRunResponse {
    id: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    report: Option<Report>,
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
            .map_err(ApiError::forbidden)?;
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

    state
        .db
        .insert_run(&id, &account.id, "running", 0, 0, "{}")?;

    let engine = state.engine.clone();
    let request = RunRequest {
        wasm,
        entry,
        limits,
        grants,
        args: Vec::new(),
    };

    if body.wait {
        let outcome =
            tokio::task::spawn_blocking(move || engine.execute_with(request, &kill)).await;
        {
            let mut guard = state.inflight.lock().await;
            guard.remove(&id);
        }
        let report = match outcome {
            Ok(Ok(report)) => report,
            Ok(Err(err)) => {
                let message = err.to_string();
                let payload = serde_json::json!({
                    "schema_version": 1,
                    "status": { "kind": "rejected", "message": message }
                });
                let _ =
                    state
                        .db
                        .update_run(&id, &account.id, "rejected", 0, 0, &payload.to_string());
                return Err(ApiError::bad(err.to_string()));
            }
            Err(err) => {
                let message = err.to_string();
                let payload = serde_json::json!({
                    "schema_version": 1,
                    "status": { "kind": "rejected", "message": message }
                });
                let _ =
                    state
                        .db
                        .update_run(&id, &account.id, "rejected", 0, 0, &payload.to_string());
                return Err(ApiError::internal(message));
            }
        };

        let status = status_kind(&report.status);
        state.db.update_run(
            &id,
            &account.id,
            status,
            report.duration_ms,
            report.fuel_used,
            &serde_json::to_string(&report).unwrap_or_else(|_| "{}".into()),
        )?;

        Ok(Json(CreateRunResponse {
            id,
            status: status.to_string(),
            report: Some(report),
        }))
    } else {
        let state_bg = state.clone();
        let account_id = account.id.clone();
        let run_id = id.clone();
        tokio::spawn(async move {
            let report =
                tokio::task::spawn_blocking(move || engine.execute_with(request, &kill)).await;
            {
                let mut guard = state_bg.inflight.lock().await;
                guard.remove(&run_id);
            }
            match report {
                Ok(Ok(report)) => {
                    let status = status_kind(&report.status);
                    let _ = state_bg.db.update_run(
                        &run_id,
                        &account_id,
                        status,
                        report.duration_ms,
                        report.fuel_used,
                        &serde_json::to_string(&report).unwrap_or_else(|_| "{}".into()),
                    );
                }
                Ok(Err(err)) => {
                    let payload = serde_json::json!({
                        "schema_version": 1,
                        "status": { "kind": "rejected", "message": err.to_string() }
                    });
                    let _ = state_bg.db.update_run(
                        &run_id,
                        &account_id,
                        "rejected",
                        0,
                        0,
                        &payload.to_string(),
                    );
                }
                Err(err) => {
                    let payload = serde_json::json!({
                        "schema_version": 1,
                        "status": { "kind": "rejected", "message": err.to_string() }
                    });
                    let _ = state_bg.db.update_run(
                        &run_id,
                        &account_id,
                        "rejected",
                        0,
                        0,
                        &payload.to_string(),
                    );
                }
            }
        });

        Ok(Json(CreateRunResponse {
            id,
            status: "running".into(),
            report: None,
        }))
    }
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
    Err(ApiError::bad("provide wasm_base64 or wasm_text".into()))
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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn test_state(limit: u32) -> Arc<AppState> {
        let dir = std::env::temp_dir().join(format!(
            "boxwren-server-test-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let db = Db::open(&dir.join("test.sqlite")).unwrap();
        Arc::new(AppState {
            db,
            engine: boxwren::Engine::new().unwrap(),
            inflight: Mutex::new(HashMap::new()),
            keywarden_url: None,
            limiter: RateLimiter::new(limit, Duration::from_secs(60)),
        })
    }

    async fn body_string(res: Response) -> String {
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn port_overrides_listen_addr() {
        let configured: SocketAddr = "127.0.0.1:8787".parse().unwrap();
        assert_eq!(
            listen_addr(configured, Some("8787")),
            "0.0.0.0:8787".parse().unwrap()
        );
        assert_eq!(listen_addr(configured, None), configured);
        assert_eq!(listen_addr(configured, Some("")), configured);
        assert_eq!(listen_addr(configured, Some("nope")), configured);
        assert_eq!(listen_addr(configured, Some("0")), configured);
    }

    #[tokio::test]
    async fn healthz_ok() {
        let app = build_app(test_state(100), &parse_cors_origins(None));
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(body_string(res).await, "ok");
    }

    #[tokio::test]
    async fn rate_limit_blocks_burst_on_v1() {
        let state = test_state(2);
        let app = build_app(state, &parse_cors_origins(None));

        for i in 0..2 {
            let res = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/v1/accounts")
                        .header("content-type", "application/json")
                        .body(Body::from(format!(r#"{{"email":"u{i}@example.com"}}"#)))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::OK, "request {i}");
        }

        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/accounts")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"email":"blocked@example.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(body_string(res).await.contains("rate limit"));
    }

    #[tokio::test]
    async fn cors_reflects_allowlisted_origin() {
        let origins = parse_cors_origins(Some("http://localhost:3000"));
        let app = build_app(test_state(100), &origins);
        let res = app
            .oneshot(
                Request::builder()
                    .method("OPTIONS")
                    .uri("/v1/me")
                    .header("origin", "http://localhost:3000")
                    .header("access-control-request-method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let allow = res
            .headers()
            .get("access-control-allow-origin")
            .and_then(|v| v.to_str().ok());
        assert_eq!(allow, Some("http://localhost:3000"));
    }
}
