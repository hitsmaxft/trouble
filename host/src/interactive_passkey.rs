//! Application-supplied passkey response for a single interactive pairing.
//!
//! This is intentionally transport-agnostic. The application decides how a
//! passkey is collected; the GATT connection only waits for submit or cancel.

use core::sync::atomic::{AtomicBool, Ordering};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

static ACTIVE: AtomicBool = AtomicBool::new(false);
static RESPONSE: Signal<CriticalSectionRawMutex, Option<u32>> = Signal::new();

/// Whether the active connection is waiting for a six-digit passkey.
pub fn is_active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

/// Submit a passkey collected by the application.
pub fn submit(passkey: u32) -> bool {
    if passkey > 999_999 || !is_active() {
        return false;
    }
    RESPONSE.signal(Some(passkey));
    true
}

/// Cancel the current interactive pairing.
pub fn cancel() -> bool {
    if !is_active() {
        return false;
    }
    RESPONSE.signal(None);
    true
}

pub(crate) fn begin() {
    RESPONSE.reset();
    ACTIVE.store(true, Ordering::Release);
}

pub(crate) fn end() {
    ACTIVE.store(false, Ordering::Release);
    RESPONSE.reset();
}

pub(crate) async fn wait() -> Option<u32> {
    RESPONSE.wait().await
}
