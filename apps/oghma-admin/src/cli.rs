use clap::Parser;

#[derive(Parser)]
#[clap(name = "Oghma Admin")]
#[clap(author = "MBRound18. <12646562+mbround18@users.noreply.github.com>")]
#[clap(about = "Manage Oghma instance", long_about = None)]
pub struct Cli {
    /// Set the host for which to interact with.
    #[clap(long, default_value_t=String::from("http://localhost:8123"), value_parser)]
    pub host: String,
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand)]
pub enum Action {
    /// Download the mutated dgraph schema.
    DownloadSchema {
        #[clap(short, long)]
        output_file: Option<String>,
    },

    /// Upload our schema to dgraph.
    UploadSchema,

    /// Fetch All Users
    GetUsers,

    /// Add a discord user via CLI
    AddUser { snowflake: String, name: String },
}
