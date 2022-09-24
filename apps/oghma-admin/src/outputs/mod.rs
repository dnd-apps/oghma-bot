use oghma_graphql::entities::DiscordUsers;

pub fn discord_users(users: DiscordUsers) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.set_header(vec!["ID", "Snowflake", "Name"]);
    for user in users.users {
        table.add_row(vec![user.id, user.snowflake, user.name]);
    }
    table
}
