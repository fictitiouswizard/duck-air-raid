#[cfg(feature = "ssr")]
use {
    actix_web::{web, Responder, web::Redirect },
    crate::auth::discord::API_ENDPOINT,
    crate::auth::discord::models::TokenExchangeBody,
    crate::server_state::AppState,
    crate::auth::discord::models::TokenExchangeResponse,
    actix_session::Session,
};
use serde::Deserialize;
use crate::auth::discord::models::User;

#[derive(Deserialize)]
pub struct DiscordCodeResponse {
    pub code: String,
}

#[cfg(feature = "ssr")]
pub async fn handle_session_test(
    session: Session
) -> impl Responder {
    if let Some(email) = session.get::<String>("test").unwrap() {
        println!("{}", email);
        return email;
    } else {
        println!("{}", "greg");
        return "greg".to_string();
    }
}

#[cfg(feature = "ssr")]
pub async fn handle_discord_code(
    app_state: web::Data<AppState>,
    info: web::Query<DiscordCodeResponse>,
    session: Session,
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
        .unwrap()//.text().await.unwrap();
        .json::<TokenExchangeResponse>()
        .await
        .unwrap();

    let auth_header = format!(
        "{} {}",
        token_info.token_type.clone(),
        token_info.access_token.clone()
    );

    let user_info = client.get(format!("{}/users/@me", API_ENDPOINT))
        .header("Authorization", auth_header)
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();
    let email = user_info.email.clone().unwrap();

    println!("{}", &email);
    session.insert("test", email).expect("unable to write to session storage");

    Redirect::to("/auth/session")
}
