use axum_template as lib;
use snafu::{ResultExt, Snafu};
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

    let config = lib::config::Config::from_env().context(LoadConfigSnafu)?;
    let state = lib::state::AppState::from_rt_config(config.runtime);
    let (router, api) = lib::route::router().with_state(state).split_for_parts();
    let router = router.merge(Scalar::with_url("/api/docs", api));

    let listen_str = config.bootstrap.listen.to_string();
    let listener = tokio::net::TcpListener::bind(config.bootstrap.listen)
        .await
        .context(NetBindSnafu {
            on: config.bootstrap.listen,
        })?;
    let serve = axum::serve(listener, router)
        .with_graceful_shutdown(ExitSignals::new().context(InitExitSignalsSnafu)?);
    tracing::info!("server started at: {}", listen_str);
    let access_url = listen_str
        .replace("0.0.0.0", "127.0.0.1")
        .replace("::", "::1");
    tracing::info!("WebUI started at http://{}", access_url);
    tracing::info!("check the api doc at: http://{}/api/docs", access_url);
    match serve.await {
        Ok(()) => tracing::info!("axum server exited."),
        Err(err) => tracing::error!("axum server error: {}", err),
    }
    Ok(())
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("initialize exit signals for graceful shutdown: {}", source))]
    InitExitSignals { source: std::io::Error },
    #[snafu(display("load configuration: {}", source))]
    LoadConfig { source: lib::config::Error },
    #[snafu(display("bind to address {}: {}", on, source))]
    NetBind {
        source: std::io::Error,
        on: std::net::SocketAddr,
    },
}

struct ExitSignals {
    sigterm: tokio::signal::unix::Signal,
    sigint: tokio::signal::unix::Signal,
}
impl ExitSignals {
    fn new() -> std::io::Result<Self> {
        let sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        let sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
        Ok(Self { sigterm, sigint })
    }
}
impl Future for ExitSignals {
    type Output = ();
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let std::task::Poll::Ready(Some(())) = self.sigterm.poll_recv(cx) {
            std::task::Poll::Ready(())
        } else if let std::task::Poll::Ready(Some(())) = self.sigint.poll_recv(cx) {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}
