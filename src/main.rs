use std::collections::HashMap;
mod git;
use dotenv::dotenv;
use git::Git;

// TODO: Change gql_client to graphql_client or reqwest with json.

#[tokio::main]
async fn main() {
    dotenv().ok();
    let endpoint =
        dotenv::var("GRAPHQL_ENDPOINT").expect("Endpoint not found");
    let mut headers = HashMap::new();
    headers.insert(
        "authorization",
        format!(
            "Bearer {}",
            dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN")
                .expect("PAC not found")
        ),
    );
    headers.insert("user-agent", "PostmanRuntime/7.29.2".to_string());
    let client =
        gql_client::Client::new_with_headers(endpoint, headers);

    Git::get_latest_commit_by_repo(
        &client,
        "git-stats-bot",
        &dotenv::var("GITHUB_USERNAME").expect("Username not found"),
        5,
    )
    .await;
    Git::get_recent_active_repos(
        &client,
        &dotenv::var("GITHUB_USERNAME").expect("Username not found"),
        5,
    )
    .await;
}
