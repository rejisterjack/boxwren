use std::path::PathBuf;

/// Failures of the embedder's request, before a guest report exists.
///
/// Guest traps, limit hits, and rejected modules are [`crate::Report`] values.
/// Those are outcomes of a run. These errors mean the run was not started.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("directory grant does not exist or is not a directory: {0}")]
    InvalidGrant(PathBuf),

    #[error("invalid grant: {0}")]
    BadGrant(String),

    #[error("kill handle was created by a different engine")]
    ForeignKillHandle,

    #[error("policy denied the run: {0}")]
    Policy(String),

    #[error(transparent)]
    Host(#[from] wasmtime::Error),
}
