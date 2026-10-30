use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub email: String,
}

#[derive(Clone, Serialize)]
pub struct ApiKeyRow {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
}

#[derive(Clone, Serialize)]
pub struct RunRow {
    pub id: String,
    pub status: String,
    pub duration_ms: u64,
    pub fuel_used: u64,
    pub created_at: String,
    #[serde(skip_serializing)]
    pub report_json: String,
}

pub struct Db {
    conn: std::sync::Mutex<Connection>,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|err| err.to_string())?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL REFERENCES accounts(id),
                name TEXT NOT NULL,
                prefix TEXT NOT NULL,
                hash TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS runs (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL REFERENCES accounts(id),
                status TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                fuel_used INTEGER NOT NULL,
                report_json TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            ",
        )
        .map_err(|err| err.to_string())?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    pub fn create_account(&self, email: &str) -> Result<(Account, String), String> {
        let id = Uuid::new_v4().to_string();
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        conn.execute(
            "INSERT INTO accounts (id, email) VALUES (?1, ?2)",
            params![id, email],
        )
        .map_err(|err| err.to_string())?;
        drop(conn);
        let (key_row, api_key) = self.create_key(&id, "default")?;
        let _ = key_row;
        Ok((
            Account {
                id,
                email: email.to_string(),
            },
            api_key,
        ))
    }

    pub fn create_key(&self, account_id: &str, name: &str) -> Result<(ApiKeyRow, String), String> {
        let id = Uuid::new_v4().to_string();
        let raw = format!("bw_{}", hex::encode(rand_bytes(24)));
        let prefix: String = raw.chars().take(10).collect();
        let hash = hash_key(&raw);
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        conn.execute(
            "INSERT INTO api_keys (id, account_id, name, prefix, hash) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, account_id, name, prefix, hash],
        )
        .map_err(|err| err.to_string())?;
        let created_at: String = conn
            .query_row(
                "SELECT created_at FROM api_keys WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;
        Ok((
            ApiKeyRow {
                id,
                name: name.to_string(),
                prefix,
                created_at,
            },
            raw,
        ))
    }

    pub fn list_keys(&self, account_id: &str) -> Result<Vec<ApiKeyRow>, String> {
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, prefix, created_at FROM api_keys WHERE account_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(|err| err.to_string())?;
        let rows = stmt
            .query_map(params![account_id], |row| {
                Ok(ApiKeyRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    prefix: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(|err| err.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| err.to_string())?;
        Ok(rows)
    }

    pub fn account_for_key(&self, api_key: &str) -> Result<Option<Account>, String> {
        let hash = hash_key(api_key);
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        conn.query_row(
            "SELECT a.id, a.email FROM accounts a
             JOIN api_keys k ON k.account_id = a.id
             WHERE k.hash = ?1",
            params![hash],
            |row| {
                Ok(Account {
                    id: row.get(0)?,
                    email: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(|err| err.to_string())
    }

    pub fn insert_run(
        &self,
        id: &str,
        account_id: &str,
        status: &str,
        duration_ms: u64,
        fuel_used: u64,
        report_json: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        conn.execute(
            "INSERT INTO runs (id, account_id, status, duration_ms, fuel_used, report_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, account_id, status, duration_ms as i64, fuel_used as i64, report_json],
        )
        .map_err(|err| err.to_string())?;
        Ok(())
    }

    pub fn list_runs(&self, account_id: &str) -> Result<Vec<RunRow>, String> {
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT id, status, duration_ms, fuel_used, created_at, report_json
                 FROM runs WHERE account_id = ?1 ORDER BY created_at DESC LIMIT 100",
            )
            .map_err(|err| err.to_string())?;
        let rows = stmt
            .query_map(params![account_id], |row| {
                Ok(RunRow {
                    id: row.get(0)?,
                    status: row.get(1)?,
                    duration_ms: row.get::<_, i64>(2)? as u64,
                    fuel_used: row.get::<_, i64>(3)? as u64,
                    created_at: row.get(4)?,
                    report_json: row.get(5)?,
                })
            })
            .map_err(|err| err.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| err.to_string())?;
        Ok(rows)
    }

    pub fn get_run(&self, account_id: &str, id: &str) -> Result<Option<RunRow>, String> {
        let conn = self.conn.lock().map_err(|err| err.to_string())?;
        conn.query_row(
            "SELECT id, status, duration_ms, fuel_used, created_at, report_json
             FROM runs WHERE account_id = ?1 AND id = ?2",
            params![account_id, id],
            |row| {
                Ok(RunRow {
                    id: row.get(0)?,
                    status: row.get(1)?,
                    duration_ms: row.get::<_, i64>(2)? as u64,
                    fuel_used: row.get::<_, i64>(3)? as u64,
                    created_at: row.get(4)?,
                    report_json: row.get(5)?,
                })
            },
        )
        .optional()
        .map_err(|err| err.to_string())
    }
}

fn hash_key(api_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    hex::encode(hasher.finalize())
}

fn rand_bytes(n: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut buf = vec![0u8; n];
    rand::rng().fill_bytes(&mut buf);
    buf
}
