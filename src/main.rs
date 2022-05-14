use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use slack_api::Client;
use std::env;

mod handlers;
mod slack_api;

pub static SLACK_CLIENT: Lazy<Client> = Lazy::new(|| {
    let oauth_token = env::var("SLACK_BOT_OAUTH_TOKEN")
        .expect("environment variable SLACK_BOT_OAUTH_TOKEN must be set");
    Client::new(oauth_token)
});

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(handlers::handle_invite)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
