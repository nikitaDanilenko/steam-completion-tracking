use crate::config::application::Application;
use crate::models::sct::generated::types::{Game, Stats};
use crate::models::steam;
use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_client;
use crate::outgoing::steam_error::SteamError;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

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
) -> Result<steam::generated::types::AccountInformation, SteamError> {
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

fn combine_games(account_information: &steam::generated::types::AccountInformation) -> Vec<Game> {
  let games_by_id: HashMap<i64, &steam::generated::types::Game> = account_information
    .rg_games
    .iter()
    .chain(account_information.rg_perfect_unowned_games.iter())
    .map(|game| (game.appid, game))
    .collect();
  let games_by_achievement_progress: HashMap<i64, &steam::generated::types::AchievementProgress> =
    account_information
      .achievement_progress
      .iter()
      .filter_map(|progress| (progress.total > 0).then_some((progress.appid, progress)))
      .collect();

  let result: Vec<Game> = games_by_achievement_progress
    .iter()
    .filter_map(|(appid, progress)| {
      progress.vetted.and_then(|v| {
        games_by_id.get(appid).map(|game| Game {
          app_id: *appid,
          name: (*game.name).to_string(),
          total_achievements: progress.total,
          unlocked_achievements: progress.unlocked,
          counting_for_steam_completion: v > 0,
        })
      })
    })
    .collect();

  result
}

fn create_stats(account_information: steam::generated::types::AccountInformation) -> Stats {
  Stats {
    games: combine_games(&account_information),
    profile_name: account_information
      .str_profile_name
      .unwrap_or(String::from("Missing Profile Name")),
    steam_id: account_information
      .str_steam_id
      .unwrap_or(String::from("Missing Steam ID")),
  }
}
