use crate::graphql::{
    AddDiscordUser, AddDiscordUserVariables, QueryDiscordUsers, QueryDiscordUsersVariables,
};
use crate::requests::query_gql;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

// --------------------------------------
// Structures
// --------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub snowflake: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordUsers {
    pub users: Vec<DiscordUser>,
}

// --------------------------------------
// Combine into Discord Users
// --------------------------------------

// type QueryDiscordUsersResponse = HashMap<String, HashMap<String, DiscordUsers>>;

impl DiscordUsers {
    pub async fn fetch(host: &str) -> DiscordUsers {
        // Structs to parse through, instead of stacking hash maps.
        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "queryDiscordUser"))]
            query_discord_user: DiscordUsers,
        }
        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body = QueryDiscordUsers::build_query(QueryDiscordUsersVariables {});
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response)
            .unwrap_or_else(|parse_error| panic!("Failed to parse response!\n\nWith Error: {parse_error}\n\nResponse Body: {response}\n\n"));
        response_json.data.query_discord_user
    }
    pub async fn add(host: &str, snowflake: String, name: String) -> DiscordUsers {
        // Structs to parse through, instead of stacking hash maps.
        #[derive(Deserialize)]
        struct DiscordUser {
            #[serde(rename(deserialize = "discordUser"))]
            discord_user: DiscordUsers,
        }
        #[derive(Deserialize)]
        struct AddUser {
            #[serde(rename(deserialize = "addDiscordUser"))]
            add_discord_user: DiscordUser,
        }
        #[derive(Deserialize)]
        struct Response {
            data: AddUser,
        }

        // Build query and get response
        let request_body = AddDiscordUser::build_query(AddDiscordUserVariables { name, snowflake });
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.add_discord_user.discord_user,
            Err(parse_error) => {
                panic!("Failed to parse users!\n\nWith Error: {parse_error}\n\nResponse Body: {response}\n\n");
            }
        }
    }
}
