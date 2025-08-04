use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
  pub major: String,
  pub minor: String,
  pub patch: String,
}
