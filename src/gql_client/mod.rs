use gql_client::Client;
use std::collections::HashMap;

pub(crate) struct CustomizedGqlClient;

impl CustomizedGqlClient {
    pub fn new_client() -> Client {
        // Initialize gql_client
        // endpoint
        // headers
        // auth token
        // user-agent

        let endpoint = dotenv::var("GRAPHQL_ENDPOINT")
            .expect("Endpoint not found");
        let mut headers = HashMap::new();
        headers.insert(
            "authorization",
            format!(
                "Bearer {}",
                dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN")
                    .expect("PAC not found")
            ),
        );
        headers.insert(
            "user-agent",
            dotenv::var("USER_AGENT").expect("User-agent not found"),
        );
        // Client
        Client::new_with_headers(endpoint, headers)
    }
}
