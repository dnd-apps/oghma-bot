use serenity::model::id::GuildId;
use std::env;

pub fn discord_token() -> String {
    env::var("DISCORD_TOKEN").expect(
        "Discord token not provided! Please add a discord bot token to the env var DISCORD_TOKEN.",
    )
}

pub fn guild_id() -> GuildId {
    GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    )
}

pub fn client_id() -> String {
    env::var("CLIENT_ID").expect(
        "Discord client id not provided! Please add a discord client id to the env var CLIENT_ID.",
    )
}

pub fn invite_url(client_id: String, permissions: u32) -> String {
    format!(
        "https://discord.com/oauth2/authorize?client_id={}&scope=bot&permissions={}",
        client_id, permissions
    )
}
