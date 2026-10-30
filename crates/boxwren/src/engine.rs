//! Shared Wasmtime engine and the per-run kill switch.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use wasmtime::{Engine as WasmEngine, EngineWeak, Module};

use crate::error::Error;

/// Process-wide Wasmtime engine. Reuse it across runs.
///
/// A background thread ticks the engine epoch about once a millisecond.
/// Each run sets a one-tick deadline and decides, on that tick, whether to
/// keep going, time out, or die because its [`KillHandle`] was fired.
#[derive(Clone)]
pub struct Engine {
    inner: Arc<EngineInner>,
}

struct EngineInner {
    wasm: WasmEngine,
    id: u64,
    modules: Mutex<HashMap<u64, Module>>,
    /// Keeps the ticker alive until this engine is dropped.
    _ticker: Ticker,
}

struct Ticker {
    stop: Arc<AtomicBool>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

impl Drop for Ticker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let thread = self
            .thread
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .take();
        if let Some(thread) = thread {
            let _ = thread.join();
        }
    }
}

impl Engine {
    pub fn new() -> Result<Self, Error> {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        let mut config = wasmtime::Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.wasm_threads(false);
        // Compilation of a hostile module is not epoch-interruptible.
        // The module size cap is the mitigation. Parallel compilation stays on
        // because it only affects compile latency of modules we already accepted.
        let wasm = WasmEngine::new(&config)?;
        let stop = Arc::new(AtomicBool::new(false));
        let weak = wasm.weak();
        let stop_thread = Arc::clone(&stop);
        let thread = std::thread::Builder::new()
            .name("boxwren-epoch".into())
            .spawn(move || epoch_ticker(weak, stop_thread))
            .expect("spawn epoch ticker");

        Ok(Self {
            inner: Arc::new(EngineInner {
                wasm,
                id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                modules: Mutex::new(HashMap::new()),
                _ticker: Ticker {
                    stop,
                    thread: Mutex::new(Some(thread)),
                },
            }),
        })
    }

    pub(crate) fn wasm(&self) -> &WasmEngine {
        &self.inner.wasm
    }

    /// A kill switch bound to this engine. Clone it into another thread.
    pub fn kill_handle(&self) -> KillHandle {
        KillHandle {
            killed: Arc::new(AtomicBool::new(false)),
            engine_id: self.inner.id,
            wasm: self.inner.wasm.clone(),
        }
    }

    pub(crate) fn same_engine(&self, kill: &KillHandle) -> bool {
        self.inner.id == kill.engine_id
    }

    pub(crate) fn module(&self, bytes: &[u8]) -> Result<Module, wasmtime::Error> {
        let key = module_key(bytes);
        if let Some(module) = self
            .inner
            .modules
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .get(&key)
            .cloned()
        {
            return Ok(module);
        }
        let module = Module::new(&self.inner.wasm, bytes)?;
        self.inner
            .modules
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .insert(key, module.clone());
        Ok(module)
    }
}

fn module_key(bytes: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.len().hash(&mut hasher);
    bytes.hash(&mut hasher);
    hasher.finish()
}

fn epoch_ticker(engine: EngineWeak, stop: Arc<AtomicBool>) {
    while !stop.load(Ordering::Relaxed) {
        if let Some(engine) = engine.upgrade() {
            engine.increment_epoch();
        } else {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Stops one run. Other runs on the same engine keep their own deadlines.
#[derive(Clone)]
pub struct KillHandle {
    killed: Arc<AtomicBool>,
    engine_id: u64,
    wasm: WasmEngine,
}

impl KillHandle {
    pub fn kill(&self) {
        self.killed.store(true, Ordering::Release);
        // The store's deadline is one tick ahead. This increment makes that
        // store enter its epoch callback, which sees the flag and traps.
        // Other stores also enter their callback and choose to continue.
        self.wasm.increment_epoch();
    }

    pub(crate) fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.killed)
    }
}
