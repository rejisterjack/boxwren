//! Captured guest stdio. Writes past the cap are dropped and flagged.

use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use tokio::io::AsyncWrite;
use wasmtime_wasi::cli::{IsTerminal, StdoutStream};

#[derive(Clone)]
pub(crate) struct Capture {
    inner: Arc<Mutex<CaptureBuf>>,
}

struct CaptureBuf {
    bytes: Vec<u8>,
    cap: usize,
    truncated: bool,
}

impl Capture {
    pub(crate) fn new(cap: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CaptureBuf {
                bytes: Vec::new(),
                cap,
                truncated: false,
            })),
        }
    }

    pub(crate) fn text(&self) -> (String, bool) {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        (
            String::from_utf8_lossy(&guard.bytes).into_owned(),
            guard.truncated,
        )
    }
}

struct CaptureWriter {
    inner: Arc<Mutex<CaptureBuf>>,
}

impl AsyncWrite for CaptureWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut guard = self
            .inner
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        let room = guard.cap.saturating_sub(guard.bytes.len());
        if room == 0 {
            guard.truncated = true;
        } else {
            let n = buf.len().min(room);
            guard.bytes.extend_from_slice(&buf[..n]);
            if n < buf.len() {
                guard.truncated = true;
            }
        }
        // Report the full length so the guest is not killed for being noisy.
        // Bytes past the cap are discarded and `truncated` is set.
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl IsTerminal for Capture {
    fn is_terminal(&self) -> bool {
        false
    }
}

impl StdoutStream for Capture {
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(CaptureWriter {
            inner: Arc::clone(&self.inner),
        })
    }
}
