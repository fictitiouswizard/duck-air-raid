use std::fmt::format;
#[cfg(feature = "ssr")]
use {
    actix_web::{web, Responder, web::Redirect },
    crate::auth::discord::API_ENDPOINT,
    crate::auth::discord::models::TokenExchangeBody,
    crate::server_state::AppState,
};
use serde::Deserialize;
use crate::auth::discord::models::TokenExchangeResponse;

#[derive(Deserialize)]
pub struct DiscordCodeResponse {
    pub code: String,
}

#[cfg(feature = "ssr")]
pub async fn handle_discord_code(
    app_state: web::Data<AppState>,
    info: web::Query<DiscordCodeResponse>
) -> impl Responder {
    let data = TokenExchangeBody {
        grant_type: String::from("authorization_code"),
        code: info.code.clone(),
        redirect_uri: app_state.discord_redirect_uri.clone(),
    };
    let client = reqwest::Client::new();
    let token_info = client.post(format!("{}/oauth2/token", API_ENDPOINT))
        .form(&data)
        .basic_auth(&app_state.discord_client_id, Some(&app_state.discord_client_secret))
        .send()
        .await
        .unwrap()
        .json::<TokenExchangeResponse>()
        .await
        .unwrap();
    web::Json(token_info)
    // Redirect::to("/")
}
