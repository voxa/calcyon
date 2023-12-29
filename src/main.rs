mod bot;
mod server;

use std::env;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Login with a bot token from the environment
    let token =
        env::var("DISCORD_TOKEN").expect("Please provide a Discord token with DISCORD_TOKEN.");

    loop {
        tokio::join!(crate::bot::start(&token), crate::server::start(&token));
    }
}
