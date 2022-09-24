use crate::commands::CommandHandler;
use crate::environment::{client_id, discord_token, invite_url};
use log::{error, info};
use serenity::prelude::GatewayIntents;
use serenity::Client;

mod commands;
mod environment;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    oghma_logger::init();

    let token = discord_token();
    let mut client = Client::builder(
        token,
        GatewayIntents::non_privileged()
            .union(GatewayIntents::MESSAGE_CONTENT)
            .union(GatewayIntents::GUILD_MEMBERS),
    )
    .event_handler(CommandHandler)
    .await
    .expect("Error creating client!");

    info!("Invite Url: {}", invite_url(client_id(), 2348875840));

    if let Err(why) = client.start_autosharded().await {
        error!("Client Error: {:?}", why)
    }
}
