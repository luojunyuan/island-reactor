use std::time::Duration;

use windows_core::IInspectable;

use crate::bindings as Xaml;

/// RAII timer wrapper; stops and unhooks on drop.
pub struct DispatcherTimer {
    timer: Xaml::DispatcherTimer,
    tick_token: i64,
}

impl DispatcherTimer {
    pub fn new<F>(interval: Duration, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(interval, true, f)
    }

    pub fn new_one_shot<F>(after: Duration, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(after, false, f)
    }

    fn build<F>(interval: Duration, repeating: bool, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        let timer = Xaml::DispatcherTimer::new()?;
        timer.SetInterval(duration_to_timespan(interval))?;

        let timer_for_cb = timer.clone();
        let callback = UiThreadCallback(f);
        let tick_handler = windows::Foundation::EventHandler::<IInspectable>::new(move |_, _| {
            if !repeating {
                let _ = timer_for_cb.Stop();
            }
            callback.call();
            Ok(())
        });
        let tick_token = timer.Tick(&tick_handler)?;
        timer.Start()?;
        Ok(Self { timer, tick_token })
    }

    pub fn stop(&self) -> windows_core::Result<()> {
        self.timer.Stop()
    }

    pub fn start(&self) -> windows_core::Result<()> {
        self.timer.Start()
    }
}

impl Drop for DispatcherTimer {
    fn drop(&mut self) {
        let _ = self.timer.Stop();
        let _ = self.timer.RemoveTick(self.tick_token);
    }
}

/// RAII handle for a `CompositionTarget::Rendering` subscription; detaches on drop.
pub struct Rendering {
    token: i64,
}

/// Subscribe `f` to `CompositionTarget::Rendering` for the current thread.
pub fn on_rendering<F>(f: F) -> windows_core::Result<Rendering>
where
    F: Fn() + 'static,
{
    let callback = UiThreadCallback(f);
    let handler = windows::Foundation::EventHandler::<IInspectable>::new(move |_, _| {
        callback.call();
        Ok(())
    });
    let token = Xaml::CompositionTarget::Rendering(&handler)?;
    Ok(Rendering { token })
}

impl Drop for Rendering {
    fn drop(&mut self) {
        let _ = Xaml::CompositionTarget::RemoveRendering(self.token);
    }
}

fn duration_to_timespan(d: Duration) -> windows_time::TimeSpan {
    windows_time::TimeSpan::try_from(d).unwrap_or(windows_time::TimeSpan::MAX)
}

struct UiThreadCallback<F>(F);

unsafe impl<F> Send for UiThreadCallback<F> {}

impl<F: Fn()> UiThreadCallback<F> {
    fn call(&self) {
        (self.0)();
    }
}
