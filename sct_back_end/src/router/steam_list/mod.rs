use crate::config::application::Application;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SteamListParameters {
  account_type: String,
  account_id: String,
}

pub async fn handle(
  State(_application): State<Application>,
  Path(steam_list_parameters): Path<SteamListParameters>,
  headers: HeaderMap,
) -> Result<Json<AccountInformation>, SteamError> {
  let steam_token = headers
    .get("steam-token")
    .and_then(|value| value.to_str().ok())
    .ok_or(SteamError::from_str("Missing steam token header"))?;

  let account_type = AccountType::from_string(&steam_list_parameters.account_type);
  let account_id = &steam_list_parameters.account_id;

  let account_information =
    steam_client::call_games_endpoint(&account_type, account_id, steam_token).await?;

  Ok(Json(account_information))
}
