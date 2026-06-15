use crate::core::{DispatchPriority, Dispatcher};

/// Render dispatcher for the island UI thread.
///
/// The XAML island host runs on the same STA thread as the Win32 message loop,
/// so render work can execute immediately. The core render host still coalesces
/// nested requests through its own state machine.
#[derive(Clone, Default)]
pub struct WinUIDispatcher;

impl WinUIDispatcher {
    pub fn for_current_thread() -> windows_core::Result<Self> {
        Ok(Self)
    }
}

impl Dispatcher for WinUIDispatcher {
    fn enqueue(&self, _priority: DispatchPriority, f: Box<dyn FnOnce()>) -> bool {
        f();
        true
    }
}
