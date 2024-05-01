#[cfg(feature = "ssr")]
use {
   actix_web::web,
    super::discord::end_points as dep,
};

#[cfg(feature = "ssr")]
pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/discord")
                    .route(web::get().to(dep::handle_discord_code))
            )
    );
}