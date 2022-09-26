use oghma_graphql::entities::{DiscordNicknames, DiscordUsers, DiscordVoiceChannels};

pub fn discord_users(users: DiscordUsers) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.set_header(vec!["ID", "Snowflake", "Name"]);
    for user in users {
        table.add_row(vec![user.id, user.snowflake, user.name]);
    }
    table
}

pub fn discord_nicknames(nicknames: DiscordNicknames) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.set_header(vec!["ID", "Snowflake", "Name", "Alias"]);
    for nick in nicknames {
        table.add_row(vec![
            nick.id,
            nick.user.snowflake,
            nick.user.name,
            nick.nickname,
        ]);
    }
    table
}

pub fn discord_voice_channels(channels: DiscordVoiceChannels) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.set_header(vec!["ID", "Snowflake", "Users"]);
    for channel in channels {
        table.add_row(vec![
            channel.id,
            channel.snowflake,
            channel
                .user_nicknames
                .iter()
                .map(|e| String::from(&e.user.snowflake))
                .collect::<Vec<String>>()
                .join(", "),
        ]);
    }
    table
}
