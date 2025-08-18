use crate::config::application::Application;
use crate::config::version::Version;
use axum::Json;
use axum::extract::State;

pub async fn handle(State(application): State<Application>) -> Json<Version> {
  Json(application.version)
}
