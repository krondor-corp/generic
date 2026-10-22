//! Process-wide tracing setup shared by every binary.

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

/// Install a compact stdout subscriber at `log_level` (overridable via
/// `RUST_LOG`) and route panics through `tracing::error!`.
///
/// Keep the returned guards alive for the life of the process — dropping
/// them flushes and closes the non-blocking writer.
pub fn init(log_level: tracing::Level) -> Vec<tracing_appender::non_blocking::WorkerGuard> {
    let mut guards = Vec::new();

    let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(stdout_guard);

    let stdout_filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env_lossy();

    let stdout_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(stdout_writer)
        .with_filter(stdout_filter);

    tracing_subscriber::registry().with(stdout_layer).init();

    std::panic::set_hook(Box::new(|panic| match panic.location() {
        Some(loc) => {
            tracing::error!(
                message = %panic,
                panic.file = loc.file(),
                panic.line = loc.line(),
                panic.column = loc.column(),
            );
        }
        None => tracing::error!(message = %panic),
    }));

    guards
}
