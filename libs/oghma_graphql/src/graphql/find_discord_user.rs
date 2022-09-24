use graphql_client::GraphQLQuery;

// --------------------------------------
// Query Discord Users - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/find_discord_user.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct FindDiscordUser;
