pub mod entities;
mod graphql;
mod requests;
mod utils;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/graphql/"]
#[include = "*.graphql"]
#[prefix = "graphql/"]
struct Asset;

fn parse(path: &str) -> String {
    let file = Asset::get(path).unwrap();
    String::from_utf8(file.data.to_vec())
        .unwrap_or_else(|_| panic!("Failed to parse {} to string!", path))
}

pub fn schema() -> String {
    parse("graphql/schema.graphql").replace("directive @id on FIELD_DEFINITION", "")
}

pub fn introspection() -> String {
    parse("graphql/introspection.graphql").replace(
        "This file was generated based on \".graphqlconfig\". Do not edit manually.",
        "Generated file! Do not edit manually!",
    )
}
