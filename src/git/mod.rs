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

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    data: Repository,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    repository: DefaultBranchRef,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DefaultBranchRef {
    default_branch_ref: Target,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    target: History,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    page_info: PageInfo,
    edges: Vec<Commit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    oid: String,
    message: String,
    author: Author,
    commit_url: String,
    committed_date: String,
    changed_files: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    has_next_page: bool,
    has_previous_page: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    name: String,
    email: String,
    date: String,
    avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vars {
    repo: String,
    owner: String,
    quantity: u16,
}

impl Git {
    pub async fn get_latest_commit_by_repo(client: Client, repo: &str, owner: &str, quantity: u16) {
        let query = Query::query_latest_commit_by_repo();
        let vars = Vars { repo: repo.to_owned(), owner: owner.to_owned(), quantity };
        let provider =
            client.query_with_vars_unwrap::<Repository, Vars>(query.as_str(), vars).await.unwrap();

        for commit in &provider.repository.default_branch_ref.target.edges {
            println!("{} | {} | {}", commit.oid, commit.message, commit.committed_date);
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
}
