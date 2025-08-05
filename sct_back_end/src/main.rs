mod config;

use crate::config::application::Application;
use axum::extract::State;
use axum::{
  Json, Router,
  routing::{get, post},
};
use config::version::Version;
use dotenv::dotenv;
use std::net::SocketAddr;

fn create_router(application: Application) -> Router {
  Router::new().route("/", get(root)).with_state(application)
}

#[tokio::main]
async fn main() {
  // Initialize tracing
  tracing_subscriber::fmt::init();

  dotenv().ok();

  let application = Application::load().unwrap();
  let app = create_router(application.clone());

  // Run the server
  let addr = SocketAddr::from(([127, 0, 0, 1], application.server.port));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  tracing::info!("listening on {}", addr);
  axum::serve(listener, app).await.unwrap();
}

async fn root(State(application): State<Application>) -> Json<Version> {
  Json(application.version)
}
