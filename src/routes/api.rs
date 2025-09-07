use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse};
use askama::Template;
use serde::Deserialize;
use serde_json::json;

use crate::{
    templates::SpotifyPartTemplate,
    websocket_channel::{ChannelsActor, Publish},
    AppState,
};
static PLAYER_ENDPOINT: &'static str = "https://api.spotify.com/v1/me/player?market=GB";
static TOKEN_ENDPOINT: &'static str = "https://accounts.spotify.com/api/token";

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "<https://github.com/devramsean0/site-v4>"
);

#[get("/api/spotify")]
pub async fn api_spotify_get(
    request: HttpRequest,
    channels: web::Data<actix::Addr<ChannelsActor>>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let spotify_client_id = std::env::var("SPOTIFY_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let spotify_client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());
    let spotify_refresh_token =
        std::env::var("SPOTIFY_REFRESH_TOKEN").unwrap_or_else(|_| "".to_string());
    let api_update_token =
        std::env::var("API_UPDATE_TOKEN").unwrap_or_else(|_| "beans".to_string());
    if request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap_or_default()
        != format!("Bearer {api_update_token}").as_str()
    {
        return HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).finish();
    }
    let basic_auth =
        base64::encode(format!("{spotify_client_id}:{spotify_client_secret}").as_str());

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap_or_default();

    let spotify_access_token = client
        .post(TOKEN_ENDPOINT)
        .header("Authorization", format!("Basic {basic_auth}").as_str())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=refresh_token&refresh_token={spotify_refresh_token}"
        ))
        .send()
        .await;
    match spotify_access_token {
        Ok(access_token_res) => {
            log::debug!("Authenticated with Spotify!");
            let access_token = access_token_res
                .json::<SpotifyRefreshTokenRes>()
                .await
                .unwrap()
                .access_token;

            let spotify_player_res = client
                .get(PLAYER_ENDPOINT)
                .bearer_auth(access_token)
                .send()
                .await;
            match spotify_player_res {
                Ok(player_res) => {
                    let player =
                        player_res
                            .json::<serde_json::Value>()
                            .await
                            .unwrap_or_else(|_| {
                                json!({
                                    "is_playing": false
                                })
                            });
                    if player.get("is_playing").unwrap().as_bool().unwrap_or(false) {
                        log::debug!("Spotify: Currently playing");
                        let item = player.get("item").unwrap_or(&serde_json::Value::Null);
                        let title = item
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let artists = item
                            .get("artists")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|a| {
                                        a.get("name")
                                            .and_then(|n| n.as_str())
                                            .map(|s| s.to_string())
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        let is_playing =
                            player.get("is_playing").unwrap().as_bool().unwrap_or(false);
                        let album = item
                            .get("album")
                            .and_then(|a| a.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let album_image_url = item
                            .get("album")
                            .and_then(|a| a.get("images"))
                            .and_then(|imgs| imgs.as_array())
                            .and_then(|arr| arr.get(0))
                            .and_then(|img| img.get("url"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let song_url = item
                            .get("external_urls")
                            .and_then(|urls| urls.get("spotify"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let device = player
                            .get("device")
                            .and_then(|d| d.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let html = SpotifyPartTemplate {
                            is_playing,
                            artists,
                            title,
                            album,
                            album_image_url,
                            song_url,
                            device,
                        }
                        .render()
                        .expect("Template should be valid");
                        channels.do_send(Publish {
                            channel: "spotify".to_string(),
                            payload: html.clone().to_string(),
                        });
                        match state
                            .store
                            .lock()
                            .unwrap()
                            .insert("spotify".to_string(), html.clone())
                        {
                            None => log::debug!("KV value updated"),
                            Some(_) => log::debug!("KV value created"),
                        };
                        return HttpResponse::Ok().status(StatusCode::OK).body(html);
                    } else {
                        log::debug!("Spotify: Nothing is playing");
                        channels.do_send(Publish {
                            channel: "spotify".to_string(),
                            payload: String::new(),
                        });
                        match state
                            .store
                            .lock()
                            .unwrap()
                            .insert("spotify".to_string(), String::new())
                        {
                            None => log::debug!("KV value updated"),
                            Some(_) => log::debug!("KV value created"),
                        };
                        return HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish();
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch spotify player: {err}");
                    channels.do_send(Publish {
                        channel: "spotify".to_string(),
                        payload: String::new(),
                    });
                    match state
                        .store
                        .lock()
                        .unwrap()
                        .insert("spotify".to_string(), String::new())
                    {
                        None => log::debug!("KV value updated"),
                        Some(_) => log::debug!("KV value created"),
                    };
                    return HttpResponse::Ok()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .finish();
                }
            }
        }
        Err(err) => {
            log::error!("Failed to authenticate spotify: {err}");
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .finish();
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct SpotifyRefreshTokenRes {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
    scope: String,
}
