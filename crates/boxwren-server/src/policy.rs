//! Optional Keywarden admission before a hosted run starts.

use serde_json::json;

pub async fn preflight_keywarden(url: &str, account_id: &str, entry: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&json!({
            "action": "boxwren.run",
            "account_id": account_id,
            "entry": entry,
        }))
        .send()
        .await
        .map_err(|err| format!("keywarden request failed: {err}"))?;
    if response.status().is_success() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    Err(format!("keywarden denied run: {body}"))
}
