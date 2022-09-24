use graphql_client::QueryBody;
use serde::Serialize;

pub async fn query_gql<T: Serialize>(host: &str, request_body: QueryBody<T>) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/graphql", &host))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send graphql request");
    res.text().await.expect("output as text failed")
}
