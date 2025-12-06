use axum::{http::StatusCode, response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = env::args().nth(1).expect("No Port given");
    let port = args.parse::<u16>().expect("Cannot parse given port");

    // Serve everything from the current directory (From the working directory you invoke the `server` binary from)
    let app = Router::new()
        .fallback_service(ServeDir::new("."))
        .route("/", get(index_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server error");
}

async fn index_handler() -> (StatusCode, Html<String>) {
    match std::fs::read_to_string("./index.html") {
        Ok(content) => (StatusCode::OK, Html(content)),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Html("index.html not found".to_string()),
        ),
    }
}
