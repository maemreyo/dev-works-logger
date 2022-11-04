mod git;
pub mod gql_client;
use dotenv::dotenv;
use git::Git;

#[tokio::main]
async fn main() {
    // Initialize env variables
    dotenv().ok();

    // Initialize GQL Client
    let client = gql_client::GqlClient::new_client();
    // Trigger action to get latest commits of repos
    let result = Git::get_latest_commits(
        &client,
        &dotenv::var("GITHUB_USERNAME").expect("Username not found"),
        Some(2),
        None,
    )
    .await
    .unwrap();
    println!("RESULT: {:?}", result);
}
