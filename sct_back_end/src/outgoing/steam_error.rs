use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

// Todo: We need multiple error types to reflect the different error kinds.
// They can have a common super-type, but their status codes will probably be different.
#[derive(Serialize)]
pub struct SteamError {
  pub error: String,
}

impl SteamError {
  pub fn from_str(message: &str) -> Self {
    SteamError {
      error: String::from(message),
    }
  }

  pub fn from_string(message: String) -> Self {
    SteamError { error: message }
  }
}

impl IntoResponse for SteamError {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}
