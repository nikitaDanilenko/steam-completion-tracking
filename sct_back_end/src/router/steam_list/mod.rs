use crate::config::application::Application;
use crate::models::sct::generated::types::Stats;
use crate::models::steam::generated::types::AccountInformation;
use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_client;
use crate::outgoing::steam_error::SteamError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamListParameters {
  account_type: String,
  account_id: String,
}

pub async fn handle(
  application: State<Application>,
  steam_list_parameters: Path<SteamListParameters>,
  headers: HeaderMap,
) -> Result<Json<Stats>, SteamError> {
  fetch_account_information(application, steam_list_parameters, headers)
    .await
    .map(|account_information| Json(create_stats(account_information)))
}

async fn fetch_account_information(
  State(_application): State<Application>,
  Path(steam_list_parameters): Path<SteamListParameters>,
  headers: HeaderMap,
) -> Result<AccountInformation, SteamError> {
  let steam_token = headers
    .get("steam-token")
    .and_then(|value| value.to_str().ok())
    .ok_or(SteamError::from_str("Missing Steam token header"))?;

  let account_type = AccountType::from_string(&steam_list_parameters.account_type);
  let account_id = &steam_list_parameters.account_id;

  let account_information =
    steam_client::call_games_endpoint(&account_type, account_id, steam_token).await?;

  Ok(account_information)
}

fn create_stats(account_information: AccountInformation) -> Stats {
  Stats {
    games: Vec::new(),
    profile_name: account_information
      .str_profile_name
      .unwrap_or(String::from("Missing Profile Name")),
    steam_id: account_information
      .str_steam_id
      .unwrap_or(String::from("Missing Steam ID")),
  }
}
