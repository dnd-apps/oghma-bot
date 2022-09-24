use log::{error, info};
use oghma_graphql::schema;
use std::process::exit;

pub async fn upload_schema(host: &str) {
    let schema_body = Vec::from(schema());
    let result = reqwest::Client::new()
        .post(format!("{host}/admin/schema"))
        .body(schema_body)
        .send()
        .await
        .unwrap_or_else(|_| {
            error!("Failed to send schema post to {host} is the server running?");
            exit(1)
        });
    info!(
        "Body: {}",
        result.text().await.expect("Failed to parse body!")
    );
}
