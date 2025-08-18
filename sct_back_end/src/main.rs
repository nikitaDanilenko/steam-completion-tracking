mod config;
mod models;
mod outgoing;
mod router;

use crate::config::application::Application;

use dotenv::dotenv;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  // Initialize tracing
  tracing_subscriber::fmt::init();

  dotenv().ok();

  let application = Application::load().unwrap();
  let app = router::create_router(application.clone());

  // Run the server
  let address = SocketAddr::from(([127, 0, 0, 1], application.server.port));
  let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

  tracing::info!("listening on {}", address);
  axum::serve(listener, app).await.unwrap();
}
