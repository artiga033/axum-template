use axum_template as lib;
use snafu::ResultExt;
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

    let config = lib::config::Config::from_env().whatever_context("load config")?;
    let state = lib::state::AppState::from_rt_config(config.runtime);
    let (router, api) = lib::route::router().with_state(state).split_for_parts();
    let router = router.merge(Scalar::with_url("/api/docs", api));

    let listen_str = config.bootstrap.listen.to_string();
    let listener = tokio::net::TcpListener::bind(config.bootstrap.listen)
        .await
        .whatever_context("bind to address")?;
    let serve = axum::serve(listener, router);
    tracing::info!("server started at: {}", listen_str);
    let access_url = listen_str
        .replace("0.0.0.0", "127.0.0.1")
        .replace("::", "::1");
    tracing::info!("WebUI started at http://{}", access_url);
    tracing::info!("check the api doc at: http://{}/api/docs", access_url);
    let quit = tokio::signal::ctrl_c();
    tokio::select! {
        _ = serve => {
            tracing::info!("Server stopped");
        }
        _ = quit => {
            tracing::info!("Graceful shutting down...");
        }
    }
    Ok(())
}
