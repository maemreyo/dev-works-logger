mod query;
use self::query::Query;
use gql_client::Client;
use serde::{Deserialize, Serialize};
/**
 * Git Provider struct
 * getRepositories
 * getContributionStats
 */

pub struct Git;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub repository: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub default_branch_ref: DefaultBranchRef,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBranchRef {
    pub target: Target,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub history: History,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub oid: String,
    pub message: String,
    pub author: Author,
    pub commit_url: String,
    pub committed_date: String,
    pub changed_files: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vars {
    repo: String,
    owner: String,
    quantity: u16,
}

impl Git {
    pub async fn get_latest_commit_by_repo(
        client: Client,
        repo: &str,
        owner: &str,
        quantity: u16,
    ) {
        let query = Query::latest_commit_by_repo();
        let vars = Vars {
            repo: repo.to_owned(),
            owner: owner.to_owned(),
            quantity,
        };
        let response = client
            .query_with_vars_unwrap::<Data, Vars>(
                query.as_str(),
                vars,
            )
            .await
            .unwrap();
        for edge in &response
            .repository
            .default_branch_ref
            .target
            .history
            .edges
        {
            println!(
                "{} | {} | {}",
                edge.node.author.name,
                edge.node.message,
                edge.node.changed_files
            );
        }
    }

    // pub fn getRepositories(owner: &str, number: u16) {
    // {
    //   repositoryOwner(login: "maemreyo") {
    //     id
    //     avatarUrl
    //     login
    //     repositories(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
    //       nodes {
    //         description
    //         id
    //         nameWithOwner
    //         updatedAt
    //         pushedAt
    //       }
    //       pageInfo {
    //         endCursor
    //         hasNextPage
    //         hasPreviousPage
    //         startCursor
    //       }
    //       totalCount
    //     }
    //   }
    // }
    //     todo!()
    // }

    // pub fn recent_active_repos() -> String {
    //     String::from(
    //         r#"query MyQuery($login: String, $first: Int = 5) {
    // 			repositoryOwner(login: $login) {
    // 			  repositories(
    // 				orderBy: {field: PUSHED_AT, direction: DESC}
    // 				isFork: false
    // 				first: $first
    // 			  ) {
    // 				edges {
    // 				  node {
    // 					name
    // 					description
    // 					pushedAt
    // 				  }
    // 				}
    // 			  }
    // 			  avatarUrl
    // 			}
    // 		  }"#,
    //     )
    // }
}
