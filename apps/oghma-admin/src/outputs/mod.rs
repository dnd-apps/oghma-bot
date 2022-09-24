use oghma_graphql::entities::{DiscordNicknames, DiscordUsers};

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
