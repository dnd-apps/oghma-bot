use crate::graphql::{
    AddDiscordUser, AddDiscordUserVariables, FindDiscordUser, FindDiscordUserVariables,
    QueryDiscordUsers, QueryDiscordUsersVariables,
};
use crate::requests::query_gql;
use crate::utils::parse_error;
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

pub type DiscordUsers = Vec<DiscordUser>;

// --------------------------------------
// Combine into Discord Users
// --------------------------------------

// type QueryDiscordUsersResponse = HashMap<String, HashMap<String, DiscordUsers>>;

impl DiscordUser {
    pub async fn find(host: &str, snowflake: String) -> DiscordUsers {
        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "queryDiscordUser"))]
            query: DiscordUsers,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body = FindDiscordUser::build_query(FindDiscordUserVariables {
            snowflake: Option::from(snowflake),
        });
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query,
            Err(err) => parse_error(err, response),
        }
    }
    pub async fn fetch(host: &str) -> DiscordUsers {
        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "queryDiscordUser"))]
            query: DiscordUsers,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body = QueryDiscordUsers::build_query(QueryDiscordUsersVariables {});
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query,
            Err(err) => parse_error(err, response),
        }
    }
    pub async fn add(host: &str, snowflake: String, name: String) -> DiscordUsers {
        // Structs to parse through, instead of stacking hash maps.
        #[derive(Deserialize)]
        struct SubQuery {
            #[serde(rename(deserialize = "discordUser"))]
            sub_query: DiscordUsers,
        }

        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "addDiscordUser"))]
            query: SubQuery,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body = AddDiscordUser::build_query(AddDiscordUserVariables { name, snowflake });
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query.sub_query,
            Err(err) => parse_error(err, response),
        }
    }
}
