use graphql_client::GraphQLQuery;

// --------------------------------------
// Query Discord Users - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/query_discord_users.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct QueryDiscordUsers;
