use leptos::*;

#[cfg(feature = "ssr")]
use {
    leptos_actix::extract,
    actix_session::Session,
};

#[server(GetMe, "/api/me")]
pub async fn get_me() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    println!("{:?}", session.status());
    if let Some(email) = session.get::<String>("test").unwrap() {
        println!("{}", email);
        return Ok(email);
    } else {
        println!("{}", "greg");
        return Ok(String::from("greg"));
    }
}