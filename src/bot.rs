use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::event::{GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent};
use serenity::model::guild::ScheduledEvent;
use serenity::prelude::*;
use serenity::Client;

#[group]
#[commands(ping)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // TODO - implement these handlers
    async fn guild_scheduled_event_create(&self, context: Context, event: ScheduledEvent) {}
    async fn guild_scheduled_event_update(&self, context: Context, event: ScheduledEvent) {}
    async fn guild_scheduled_event_delete(&self, context: Context, event: ScheduledEvent) {}
    async fn guild_scheduled_event_user_add(
        &self,
        context: Context,
        subscribed: GuildScheduledEventUserAddEvent,
    ) {
    }
    async fn guild_scheduled_event_user_remove(
        &self,
        context: Context,
        unsubscribed: GuildScheduledEventUserRemoveEvent,
    ) {
    }
}

// ~calendar
// ~calendar edit
// ~calendar create
// ~calendar delete
// ~calendar add-guild
// ~calendar remove-guild

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

pub async fn start(token: String) {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // Prefix of the bot is '~'
        .group(&GENERAL_GROUP);

    // TODO - allow gateway intents to get guild scheduled events, guild users
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
