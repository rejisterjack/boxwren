//! Optional admission hooks. Policy belongs to Keywarden; Boxwren only asks
//! whether a run may start.

use crate::error::Error;
use crate::run::RunRequest;

/// Called before a module is instantiated. Deny to refuse the run.
pub trait Preflight: Send + Sync {
    fn allow(&self, request: &RunRequest) -> Result<(), PolicyDeny>;
}

/// A preflight denial. Surfaces as [`Error::Policy`] before the guest starts.
#[derive(Debug, Clone)]
pub struct PolicyDeny {
    pub message: String,
}

impl PolicyDeny {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<PolicyDeny> for Error {
    fn from(value: PolicyDeny) -> Self {
        Error::Policy(value.message)
    }
}
