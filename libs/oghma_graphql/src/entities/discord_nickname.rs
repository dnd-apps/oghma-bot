use crate::entities::DiscordUser;
use crate::graphql::{
    AddDiscordNickname, AddDiscordNicknameVariables, DiscordUserNicknameRef, QueryDiscordNicknames,
    QueryDiscordNicknamesVariables,
};
use crate::requests::query_gql;
use crate::utils::parse_error;
use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DiscordNickname {
    pub id: String,
    pub user: DiscordUser,
    pub nickname: String,
}
pub type DiscordNicknames = Vec<DiscordNickname>;

impl DiscordNickname {
    pub async fn fetch(host: &str) -> DiscordNicknames {
        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "queryDiscordNickname"))]
            query: DiscordNicknames,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        // Build query and get response
        let request_body = QueryDiscordNicknames::build_query(QueryDiscordNicknamesVariables {});
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query,
            Err(err) => parse_error(err, response),
        }
    }
    pub async fn add(host: &str, snowflake: String, name: String) -> DiscordNicknames {
        // Structs to parse through, instead of stacking hash maps.
        #[derive(Deserialize)]
        struct SubQuery {
            #[serde(rename(deserialize = "discordNickname"))]
            sub_query: DiscordNicknames,
        }

        #[derive(Deserialize)]
        struct Query {
            #[serde(rename(deserialize = "addDiscordNickname"))]
            query: SubQuery,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Query,
        }

        let users = DiscordUser::find(&host, snowflake.clone()).await;

        if users.iter().count().eq(&0) {
            panic!("No user found with snowflake: {snowflake}");
        }

        let user = &users[0];

        let user = DiscordUserNicknameRef {
            id: Option::from(String::from(&user.id)),
            snowflake: Option::from(String::from(&user.snowflake)),
            name: Option::from(String::from(&user.name)),
        };

        // Build query and get response
        let request_body =
            AddDiscordNickname::build_query(AddDiscordNicknameVariables { name, user });
        let response = query_gql(host, request_body).await;

        // Parse response
        let response_json = serde_json::from_str::<Response>(&response);
        match response_json {
            Ok(res) => res.data.query.sub_query,
            Err(err) => parse_error(err, response),
        }
    }
}
