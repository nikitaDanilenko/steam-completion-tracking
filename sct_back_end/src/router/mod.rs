use crate::config::application::Application;
use axum::http::{HeaderName, Method};
use axum::routing::get;
use axum::{Router, http};
use tower_http::cors::{Any, CorsLayer};

mod root;
mod steam_list; // Add a new module for Steam OpenID authentication

pub fn create_router(application: Application) -> Router {
  let cors = CorsLayer::new()
    .allow_origin(
      "http://localhost:5173"
        .parse::<http::HeaderValue>()
        .unwrap(),
    )
    .allow_methods([Method::GET])
    .allow_headers([
      http::header::CONTENT_TYPE,
      HeaderName::from_static("steam-token"),
    ]);

  Router::new()
    .route("/", get(root::handle))
    .route(
      "/steam-list/{account_type}/{account_id}",
      get(steam_list::handle),
    )
    .layer(cors)
    .with_state(application)
}
