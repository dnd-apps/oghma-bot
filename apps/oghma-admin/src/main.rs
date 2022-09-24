use crate::cli::Action;
use crate::outputs::discord_users;
use clap::Parser;
use log::info;
use oghma_graphql::entities::DiscordUsers;

mod cli;
mod commands;
mod outputs;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    oghma_logger::init();
    let cli = cli::Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.action {
        Action::DownloadSchema { output_file } => {
            commands::download_schema(output_file.to_owned()).await
        }
        Action::UploadSchema => commands::upload_schema(&cli.host).await,
        Action::GetUsers => {
            let users = DiscordUsers::fetch(&cli.host).await;
            info!("\n{}", discord_users(users))
        }
        Action::AddUser { snowflake, name } => {
            let users = DiscordUsers::add(&cli.host, snowflake.to_owned(), name.to_owned()).await;
            info!("\n{}", discord_users(users))
        }
    };
}
