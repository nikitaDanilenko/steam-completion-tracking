use crate::config::application::Application;
use axum::Router;
use axum::routing::get;

pub mod root;
pub mod routes;
pub mod steam_list;

pub fn create_router(application: Application) -> Router {
  Router::new()
    .route("/", get(root::handle))
    .route(
      "/steam-list/{account_type}/{account_id}",
      get(steam_list::handle),
    )
    .with_state(application)
}
