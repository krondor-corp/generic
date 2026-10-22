use generic_rust_app::config::Config;
use generic_rust_app::http::HttpServer;
use generic_rust_app::logging;
use generic_rust_app::runtime::{Service, ShutdownHandle};
use generic_rust_app::state::AppState;
use generic_rust_app::tasks::TaskWorker;

#[tokio::main]
async fn main() {
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            std::process::exit(1);
        }
    };

    let _guards = logging::init(config.log_level);

    let state = match AppState::from_config(&config).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("failed to create app state: {e}");
            std::process::exit(3);
        }
    };

    let (mut handle, shutdown_rx) = ShutdownHandle::new();

    handle.push(HttpServer::spawn(state.clone(), shutdown_rx.clone()));
    handle.push(TaskWorker::spawn(state.clone(), shutdown_rx.clone()));

    handle.wait().await;
}
