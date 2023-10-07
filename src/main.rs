use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

use calcyon::help_message::*;
use calcyon::show_profile::*;
use calcyon::subscribe_user::*;

#[group]
#[commands(help, profile, subscribe)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // Prefix of the bot is '~'
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// Help message command
#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx,help_message()).await?;

    Ok(())
}

// Show user their linked profile/Tell them to set it up
#[command]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx,show_profile(ctx)).await?;
    
    Ok(())
}

// Subscribe to the current channel
#[command]
async fn subscribe(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, subscribe_user(ctx)).await?;

    Ok(())
}