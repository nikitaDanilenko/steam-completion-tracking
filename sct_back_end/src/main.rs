mod config;
mod models;
mod outgoing;
// todo: Restructure. The main.rs should only start the server, and have no own logic.
use crate::config::application::Application;
use axum::extract::{Path, State};
use axum::{
  Json, Router,
  routing::{get, post},
};
use config::version::Version;
use dotenv::dotenv;

use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_client;
use axum::http::HeaderMap;
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

fn extract_list_of_games(html: &str) -> Option<String> {
  // Parse the HTML document
  let document = Html::parse_document(html);

  // Create a selector for the element with id "gameslist_config".
  // The element (template) with this id contains an attribute with the list of games.
  let selector = Selector::parse("#gameslist_config").unwrap();

  // Find the element and extract the attribute mentioned above
  document
    .select(&selector)
    .next()
    .and_then(|element| element.value().attr("data-profile-gameslist"))
    .map(|data| data.replace("&quot;", "\"").to_string())
}

async fn steam_list(
  State(_application): State<Application>,
  Path(steam_list_parameters): Path<SteamListParameters>,
  headers: HeaderMap,
) -> Json<serde_json::Value> {
  // Todo: Handle gracefully
  let steam_token = headers.get("steam-token").unwrap();
  // Todo: Tidy up the unwrapping - it should be more consistent, and not panic.
  let account_type = AccountType::from_string(&steam_list_parameters.account_type);
  let account_id = &steam_list_parameters.account_id;
  let response =
    steam_client::call_games_endpoint(&account_type, account_id, steam_token.to_str().unwrap())
      .await
      .unwrap_or(String::from("error"));

  let games =
    extract_list_of_games(&response).unwrap_or_else(|| String::from("HTML extraction failed"));

  Json(serde_json::from_str::<serde_json::Value>(&games[..]).unwrap())
}
