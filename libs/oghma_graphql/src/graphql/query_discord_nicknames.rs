use graphql_client::GraphQLQuery;

// --------------------------------------
// Query Discord Nicknames - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/query_discord_nicknames.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct QueryDiscordNicknames;
