mod config;
mod models;
mod outgoing;
// todo: Restructure. The main.rs should only start the server, and have no own logic.
use crate::config::application::Application;
use axum::extract::{Path, State};
use axum::{
  Json, Router, middleware,
  routing::{get, post},
};
use config::version::Version;
use dotenv::dotenv;

use crate::models::steam::generated::types::AccountInformation;
use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_client;
use crate::outgoing::steam_error::SteamError;
use axum::body::Body;
use axum::http::{Error, HeaderMap, Response};
use futures::TryFutureExt;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::debug;

#[derive(Deserialize, Debug)]
struct SteamListParameters {
  account_type: String,
  account_id: String,
}

fn create_router(application: Application) -> Router {
  Router::new()
    .route("/", get(root))
    .route("/steam-list/{account_type}/{account_id}", get(steam_list))
    .with_state(application)
}

#[tokio::main]
async fn main() {
  // Initialize tracing
  tracing_subscriber::fmt::init();

  dotenv().ok();

  let application = Application::load().unwrap();
  let app = create_router(application.clone());

  // Run the server
  let address = SocketAddr::from(([127, 0, 0, 1], application.server.port));
  let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

  tracing::info!("listening on {}", address);
  axum::serve(listener, app).await.unwrap();
}

async fn root(State(application): State<Application>) -> Json<Version> {
  Json(application.version)
}

// Todo: This function is probably unnecessary.
async fn steam_list(
  State(_application): State<Application>,
  path_parameters: Path<SteamListParameters>,
  headers: HeaderMap,
) -> Result<Json<AccountInformation>, SteamError> {
  let account_information = steam_list_internal(path_parameters, headers).await?;

  Ok(Json(account_information))
}

async fn steam_list_internal(
  Path(steam_list_parameters): Path<SteamListParameters>,
  headers: HeaderMap,
) -> Result<AccountInformation, SteamError> {
  let steam_token = headers
    .get("steam-token")
    .and_then(|value| value.to_str().ok())
    .ok_or(SteamError::from_str("Missing steam token header"))?;

  let account_type = AccountType::from_string(&steam_list_parameters.account_type);
  let account_id = &steam_list_parameters.account_id;

  let account_information =
    steam_client::call_games_endpoint(&account_type, account_id, steam_token).await?;

  Ok(account_information)
}
