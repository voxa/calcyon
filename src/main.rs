use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use serenity::Client;

#[group]
#[commands(ping)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

// ~calendar
// ~calendar edit
// ~calendar create
// ~calendar delete
// ~calendar add-guild
// ~calendar remove-guild

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // Prefix of the bot is '~'
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");

    // TODO - allow gateway intents to get guild scheduled events, guild users
    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        // On guild add Calcyon
        // Get all events from guild
        // Get all users in guild
        // For users with Calcyon calendar
        // Add event to database

        // On new event in guild
        // Get all users in guild
        // For users with Calcyon calendar
        // Add event to database

        // On user leave guild
        
        // On guild remove Calcyon

        // On use of ~calendar command
        // Get user ID
        // Generate token (UUID)
        // Create calendar entry in database
        // Reply with link

        // On /calendar link call

        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
