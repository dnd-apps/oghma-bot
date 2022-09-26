use graphql_client::GraphQLQuery;

// --------------------------------------
// Add Discord User - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection.graphql",
    query_path = "src/graphql/discord_voice_channel.graphql",
    response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct AddDiscordVoiceChannel;


// --------------------------------------
// Query Discord Voice Channels - GQL
// --------------------------------------

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/graphql/introspection.graphql",
query_path = "src/graphql/discord_voice_channel.graphql",
response_derives = "Serialize,PartialEq,Eq,Debug"
)]
pub struct QueryDiscordVoiceChannels;
