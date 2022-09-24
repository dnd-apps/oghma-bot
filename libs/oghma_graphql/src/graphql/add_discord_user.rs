use graphql_client::GraphQLQuery;

// --------------------------------------
// Add Discord User - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/add_discord_user.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct AddDiscordUser;
