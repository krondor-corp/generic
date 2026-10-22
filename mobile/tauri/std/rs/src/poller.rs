//! A named foreground loop.
//!
//! Runs `f(app)` every `interval`, immediately on [`Poller::trigger`],
//! and stops on [`Poller::stop`] or when the handle is dropped. Errors
//! from `f` are logged and the loop continues; it is the app's job to
//! surface them (emit an event, store a "last error" setting).
//!
//! This is the on-device half of the sync pattern: the server owns the
//! expensive, always-on work (scraping, in an Apalis task); the device
//! only asks "anything new since my cursor?" while it is running. Mobile
//! OSes suspend the process in the background, so a `tokio::interval`
//! here is a *foreground* guarantee only.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Runtime};
use tokio::sync::{watch, Notify};

pub struct Poller {
    name: &'static str,
    trigger: Arc<Notify>,
    stop: watch::Sender<bool>,
}

impl Poller {
    pub fn spawn<R, F, Fut>(app: AppHandle<R>, name: &'static str, interval: Duration, f: F) -> Self
    where
        R: Runtime,
        F: Fn(AppHandle<R>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        let trigger = Arc::new(Notify::new());
        let (stop, mut stopped) = watch::channel(false);

        let worker_trigger = trigger.clone();
        tauri::async_runtime::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            tracing::info!(poller = name, ?interval, "poller started");
            loop {
                tokio::select! {
                    _ = ticker.tick() => {}
                    _ = worker_trigger.notified() => { ticker.reset(); }
                    _ = stopped.changed() => {
                        if *stopped.borrow() {
                            tracing::info!(poller = name, "poller stopped");
                            return;
                        }
                    }
                }
                if let Err(e) = f(app.clone()).await {
                    tracing::warn!(poller = name, error = %e, "poll failed");
                }
            }
        });

        Self {
            name,
            trigger,
            stop,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Run `f` now (and restart the interval from now).
    pub fn trigger(&self) {
        self.trigger.notify_one();
    }

    pub fn stop(&self) {
        let _ = self.stop.send(true);
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        self.stop();
    }
}
