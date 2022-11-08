#![allow(unused)]
pub mod models;
pub mod utils;
use anyhow::Result;
use gql_client::Client;
use log::{debug, info};

use self::models::commits_in_a_time::Data as CommitsByTimeDataModel;
use self::models::latest_commit::Data as LatestCommitDataModel;
use self::models::query::{
    CommitsByTimeVars, LatestCommitVars, Query, RecentCommitVars,
};
use self::models::recent_active_repos::Data as RecentActiveReposDataModel;

/**
 * Git Provider struct
 * getRepositories
 * getContributionStats
 */

pub struct Git;

#[derive(Debug)]
pub struct Commit {
    author: String,
    pub message_headline: String,
    pub commit_url: String,
    pub committed_date: String,
    pub changed_files: u64,
}

#[derive(Debug)]
pub struct Repo {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug)]
pub struct LatestCommits {
    repo: String,
    description: String,
    url: String,
    commits: Vec<Commit>,
}

impl Git {
    const DEFAULT_RECENT_REPOS: u16 = 1;
    const DEFAULT_LATEST_COMMITS: u16 = 5;

    /**
     * Get recent commits by repo name
     */
    pub async fn get_latest_commit_by_repo(
        client: &Client,
        repo: &str,
        owner: &str,
        quantity: Option<u16>,
    ) -> Result<Vec<Commit>> {
        // Fetch Github data from GraphQL
        let query = Query::latest_commit_by_repo();
        let vars = LatestCommitVars {
            repo: repo.to_owned(),
            owner: owner.to_owned(),
            quantity: quantity.unwrap_or(Git::DEFAULT_LATEST_COMMITS),
        };
        let response = client
            .query_with_vars_unwrap::<LatestCommitDataModel, LatestCommitVars>(
                query.as_str(),
                vars,
            )
            .await
            .unwrap();

        // Build the returned result
        let mut commits: Vec<Commit> = vec![];
        for edge in &response
            .repository
            .default_branch_ref
            .target
            .history
            .edges
        {
            let data = &edge.node;
            commits.push(Commit {
                author: data.author.name.to_owned(),
                message_headline: data.message_headline.to_owned(),
                commit_url: data.commit_url.to_owned(),
                committed_date: data.committed_date.to_owned(),
                changed_files: data.changed_files,
            })
        }
        Ok(commits)
    }

    /**
     * Get recent active repos
     */
    pub async fn get_recent_active_repos(
        client: &Client,
        owner: &str,
        quantity: Option<u16>,
    ) -> Result<Vec<Repo>> {
        // Fetch Github data from GraphQL
        let query = Query::recent_active_repos();
        let vars = RecentCommitVars {
            owner: owner.to_owned(),
            quantity: quantity.unwrap_or(Git::DEFAULT_RECENT_REPOS),
        };
        let response = client
            .query_with_vars_unwrap::<RecentActiveReposDataModel, RecentCommitVars>(
                query.as_str(),
                vars,
            )
            .await
            .unwrap();

        // Build the returned result
        let mut repos: Vec<Repo> = vec![];
        for edge in &response.repository_owner.repositories.edges {
            let data = &edge.node;

            repos.push(Repo {
                name: data.name.to_owned(),
                description: data.description.to_owned(),
                url: data.url.to_owned(),
            })
        }

        Ok(repos)
    }

    /**
     * Get recent commits
     */
    pub async fn get_latest_commits(
        client: &Client,
        owner: &str,
        repo_quantity: Option<u16>,
        commit_quantity: Option<u16>,
    ) -> Result<Vec<LatestCommits>> {
        let mut latest_commits: Vec<LatestCommits> = vec![];
        // Fetch recent repos
        let recent_repos = Git::get_recent_active_repos(
            client,
            owner,
            repo_quantity,
        )
        .await
        .unwrap();

        for repo in recent_repos {
            // Fetch recent commits by repo
            let commits_by_repo = Git::get_latest_commit_by_repo(
                client,
                &repo.name,
                owner,
                commit_quantity,
            )
            .await
            .unwrap();

            latest_commits.push(LatestCommits {
                repo: repo.name.to_owned(),
                description: repo.description.to_owned(),
                url: repo.url.to_owned(),
                commits: commits_by_repo,
            })
        }
        Ok(latest_commits)
    }

    pub async fn get_commits_in_a_time(
        client: &Client,
        repo: &str,
        owner: &str,
        branch: &str,
        since: &str,
        until: &str,
    ) -> Result<(Vec<Commit>, Repo)> {
        let mut commits: Vec<Commit> = vec![];

        let query = Query::commits_in_a_day();
        let vars = CommitsByTimeVars {
            repo: repo.to_owned(),
            owner: owner.to_owned(),
            branch: branch.to_owned(),
            since: since.to_owned(),
            until: until.to_owned(),
        };
        let response = client
          .query_with_vars_unwrap::<CommitsByTimeDataModel, CommitsByTimeVars>(
              query.as_str(),
              vars,
          )
          .await
          .unwrap();

        for edge in response.repository.object.history.edges.clone() {
            commits.push(Commit {
                author: edge.node.author.name,
                message_headline: edge.node.message_headline,
                commit_url: edge.node.url,
                committed_date: edge.node.committed_date,
                changed_files: edge.node.changed_files,
            })
        }
        let repo_detail = response.repository.clone();
        let repo = Repo {
            name: repo_detail.name,
            description: repo_detail.description,
            url: repo_detail.url,
        };
        Ok((commits, repo))
    }
}
