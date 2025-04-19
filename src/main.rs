use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use tower_http::services::ServeDir;
use tokio::signal;

async fn test() -> Json<i32> {
    Json(123)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("client/dist/assets"))
        .route_service("/", ServeDir::new("client/dist"))
        .route("/api/test", get(test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}