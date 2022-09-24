use graphql_client::GraphQLQuery;

// --------------------------------------
// Add Discord Nickname - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/add_discord_nickname.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct AddDiscordNickname;
