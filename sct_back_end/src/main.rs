mod version;
use axum::{
  Json, Router,
  routing::{get, post},
};
use std::net::SocketAddr;
use version::Version;

fn create_router() -> Router {
  Router::new().route("/", get(root))
}

#[tokio::main]
async fn main() {
  // Initialize tracing
  tracing_subscriber::fmt::init();

  let app = create_router();

  // Run the server
  let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  tracing::info!("listening on {}", addr);
  axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Version> {
  Json(Version {
    major: String::from("0"),
    minor: String::from("0"),
    patch: String::from("1"),
  })
}
