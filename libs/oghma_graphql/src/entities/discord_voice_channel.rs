use crate::entities::DiscordNicknames;
use crate::graphql::{
    AddDiscordVoiceChannel, AddDiscordVoiceChannelVariables, QueryDiscordVoiceChannels,
    QueryDiscordVoiceChannelsVariables,
};
use crate::requests::query_gql;
use crate::utils::parse_error;
use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DiscordVoiceChannel {
    pub id: String,
    pub snowflake: String,
    pub user_nicknames: DiscordNicknames,
}
pub type DiscordVoiceChannels = Vec<DiscordVoiceChannel>;

impl DiscordVoiceChannel {
    pub async fn fetch(host: &str) -> DiscordVoiceChannels {
        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "queryDiscordVoiceChannel"))]
            query: DiscordVoiceChannels,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body =
            QueryDiscordVoiceChannels::build_query(QueryDiscordVoiceChannelsVariables {});
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query,
            Err(err) => parse_error(err, response),
        }
    }

    pub async fn add(host: &str, snowflake: String) -> DiscordVoiceChannels {
        // Structs to parse through, instead of stacking hash maps.
        #[derive(Deserialize)]
        struct SubQuery {
            #[serde(rename(deserialize = "discordVoiceChannel"))]
            sub_query: DiscordVoiceChannels,
        }

        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "addDiscordVoiceChannel"))]
            query: SubQuery,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body =
            AddDiscordVoiceChannel::build_query(AddDiscordVoiceChannelVariables { snowflake });
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query.sub_query,
            Err(err) => parse_error(err, response),
        }
    }
}
