
#[cfg(feature = "ssr")]
use actix_web::{web, Responder, web::Redirect };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscordCodeResponse {
    pub code: String,
}

#[cfg(feature = "ssr")]
pub async fn handle_discord_code(info: web::Query<DiscordCodeResponse>) -> impl Responder {
    String::from(info.code.clone())

    // Redirect::to("/")
}
