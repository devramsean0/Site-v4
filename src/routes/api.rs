use actix_web::{get, http::StatusCode, HttpResponse};
use serde::Deserialize;
static PLAYER_ENDPOINT: &'static str = "https://api.spotify.com/v1/me/player?market=GB";
static TOKEN_ENDPOINT: &'static str = "https://accounts.spotify.com/api/token";

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "<https://github.com/devramsean0/site-v4>"
);

#[get("/api/spotify")]
pub async fn api_spotify_get() -> HttpResponse {
    let spotify_client_id = std::env::var("SPOTIFY_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let spotify_client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());
    let spotify_refresh_token =
        std::env::var("SPOTIFY_REFRESH_TOKEN").unwrap_or_else(|_| "".to_string());

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
                    let player = player_res.json::<serde_json::Value>().await.unwrap();
                    if player.get("is_playing").unwrap().as_bool().unwrap_or(false) {
                        log::debug!("Spotify: Currently playing");
                        return HttpResponse::Ok().status(StatusCode::OK).finish();
                    } else {
                        log::debug!("Spotify: Nothing is playing");
                        return HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish();
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch spotify player: {err}");
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
