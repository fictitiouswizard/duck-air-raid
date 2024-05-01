#[cfg(feature = "ssr")]
use {
    sqlx::PgPool,
    std::env,
};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_response_type: String,
    pub discord_redirect_uri: String,
    pub discord_scopes: String,
}

#[cfg(feature = "ssr")]
impl AppState {
    pub async fn build() -> AppState {
        let db_username = env::var("DB_USERNAME").expect("Unable to find DB_USERNAME env var");
        let db_password = env::var("DB_PASSWORD").expect("unable to find DB_PASSWORD env var");
        let db_server = env::var("DB_SERVER").expect("unable to find DB_SERVER env var");
        let db_port = env::var("DB_PORT").expect("unable to find DB_PORT env var");
        let db_name = env::var("DB_NAME").expect("unable to find DB_NAME env var");
        let discord_client_id = env::var("DISCORD_CLIENT_ID").expect("unable to find discord client id");
        let discord_client_secret = env::var("DISCORD_CLIENT_SECRET").expect("unable to find discord client secret");
        let discord_response_type = env::var("DISCORD_RESPONSE_TYPE").expect("unable to find discord response type");
        let discord_redirect_uri = env::var("DISCORD_REDIRECT_URI").expect("unable to find discord redirect uri");
        let discord_scopes = env::var("DISCORD_SCOPES").expect("unable to find discord scopes");

        let postgres_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_username,
            db_password,
            db_server,
            db_port,
            db_name
        );
        AppState{
            db: PgPool::connect(&postgres_url).await.expect("Unable to connect to db"),
            discord_client_id,
            discord_client_secret,
            discord_response_type,
            discord_redirect_uri,
            discord_scopes,
        }
    }
}