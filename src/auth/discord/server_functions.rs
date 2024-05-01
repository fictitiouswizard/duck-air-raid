use leptos::*;

#[server(DiscordLogin, "/api/auth/discord")]
pub async fn send_discord_login() -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::server_state::AppState;
    use actix_web::web;

    let app_state: web::Data<AppState> = extract().await?;
    let redirect_uri = format!(
        "https://discord.com/oauth2/authorize?client_id={}&response_type={}&redirect_uri={}&scope={}",
        app_state.discord_client_id,
        app_state.discord_response_type,
        app_state.discord_redirect_uri,
        app_state.discord_scopes,
    );

    leptos_actix::redirect(&redirect_uri);
    Ok(())
}