//! Resource limiter that records the memory high-water mark and a denial.

use wasmtime::{ResourceLimiter, Result, StoreLimits, StoreLimitsBuilder};

use crate::limits::{MAX_INSTANCES, MAX_MEMORIES, MAX_TABLES};

pub(crate) struct RunLimiter {
    inner: StoreLimits,
    pub peak_memory: usize,
    pub memory_denied: bool,
}

impl RunLimiter {
    pub(crate) fn new(memory_bytes: usize) -> Self {
        let inner = StoreLimitsBuilder::new()
            .memory_size(memory_bytes)
            .instances(MAX_INSTANCES)
            .memories(MAX_MEMORIES)
            .tables(MAX_TABLES)
            .build();
        Self {
            inner,
            peak_memory: 0,
            memory_denied: false,
        }
    }
}

impl ResourceLimiter for RunLimiter {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool> {
        self.peak_memory = self.peak_memory.max(current);
        let allowed = ResourceLimiter::memory_growing(&mut self.inner, current, desired, maximum)?;
        if allowed {
            self.peak_memory = self.peak_memory.max(desired);
        } else {
            self.memory_denied = true;
        }
        Ok(allowed)
    }

    fn table_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool> {
        ResourceLimiter::table_growing(&mut self.inner, current, desired, maximum)
    }

    fn memory_grow_failed(&mut self, error: wasmtime::Error) -> Result<()> {
        self.memory_denied = true;
        ResourceLimiter::memory_grow_failed(&mut self.inner, error)
    }

    fn instances(&self) -> usize {
        ResourceLimiter::instances(&self.inner)
    }

    fn tables(&self) -> usize {
        ResourceLimiter::tables(&self.inner)
    }

    fn memories(&self) -> usize {
        ResourceLimiter::memories(&self.inner)
    }
}
