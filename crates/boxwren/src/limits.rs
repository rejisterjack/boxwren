use std::time::Duration;

/// Resource caps for a single run. Defaults deny the guest room to run away,
/// and grant it no filesystem, network, or environment access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Wasm instruction budget. This is not milliseconds.
    pub fuel: u64,
    /// Maximum size of each linear memory, in bytes.
    pub memory_bytes: usize,
    /// Wall-clock deadline. The epoch ticker enforces this.
    pub wall_time: Duration,
    /// Bytes of stdout and of stderr retained in the report.
    pub max_stdio_bytes: usize,
    /// Modules larger than this are rejected before compilation.
    pub max_module_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            fuel: DEFAULT_FUEL,
            memory_bytes: DEFAULT_MEMORY_BYTES,
            wall_time: Duration::from_secs(DEFAULT_WALL_SECS),
            max_stdio_bytes: DEFAULT_STDIO_BYTES,
            max_module_bytes: DEFAULT_MODULE_BYTES,
        }
    }
}

pub const DEFAULT_FUEL: u64 = 1_000_000_000;
pub const DEFAULT_MEMORY_BYTES: usize = 32 * 1024 * 1024;
pub const DEFAULT_WALL_SECS: u64 = 2;
pub const DEFAULT_STDIO_BYTES: usize = 1024 * 1024;
pub const DEFAULT_MODULE_BYTES: usize = 32 * 1024 * 1024;

/// Instance, memory, and table counts per store. The memory byte cap still
/// applies to each linear memory. These stop a module from multiplying memories.
pub(crate) const MAX_INSTANCES: usize = 8;
pub(crate) const MAX_MEMORIES: usize = 4;
pub(crate) const MAX_TABLES: usize = 8;
