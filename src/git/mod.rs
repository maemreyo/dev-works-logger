pub mod models;
use gql_client::Client;

use self::models::latest_commit::Data as LatestCommitDataModel;
use self::models::query::{
    LatestCommitVars, Query, RecentCommitVars,
};
use self::models::recent_active_repos::Data as RecentActiveReposDataModel;

/**
 * Git Provider struct
 * getRepositories
 * getContributionStats
 */

pub struct Git;

impl Git {
    pub async fn get_latest_commit_by_repo(
        client: &Client,
        repo: &str,
        owner: &str,
        quantity: u16,
    ) {
        let query = Query::latest_commit_by_repo();
        let vars = LatestCommitVars {
            repo: repo.to_owned(),
            owner: owner.to_owned(),
            quantity,
        };
        let response = client
            .query_with_vars_unwrap::<LatestCommitDataModel, LatestCommitVars>(
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

    pub async fn get_recent_active_repos(
        client: &Client,
        user: &str,
        quantity: u16,
    ) {
        let query = Query::recent_active_repos();
        let vars =
            RecentCommitVars { user: user.to_owned(), quantity };
        let response = client
            .query_with_vars_unwrap::<RecentActiveReposDataModel, RecentCommitVars>(
                query.as_str(),
                vars,
            )
            .await
            .unwrap();

        for edge in &response.repository_owner.repositories.edges {
            println!(
                "{} | {} | {}",
                edge.node.name,
                edge.node.description,
                edge.node.pushed_at,
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
}
