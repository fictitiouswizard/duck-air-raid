use leptos::*;
use super::server_functions::send_discord_login;

#[component]
pub fn DiscordLoginButton() -> impl IntoView {

    let click_handler = move |_|  {
        spawn_local(async {
            let _ = send_discord_login().await;
        });
    };

    view! {
        <button on:click=click_handler>Login</button>
    }
}