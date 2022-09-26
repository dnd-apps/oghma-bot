use graphql_client::GraphQLQuery;

// --------------------------------------
// Add Discord User - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/discord_user.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct AddDiscordUser;

// --------------------------------------
// Find Discord User - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/graphql/introspection.graphql",
query_path = "src/graphql/discord_user.graphql",
response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct FindDiscordUser;

// --------------------------------------
// Query Discord Users - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/graphql/introspection.graphql",
query_path = "src/graphql/discord_user.graphql",
response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct QueryDiscordUsers;
