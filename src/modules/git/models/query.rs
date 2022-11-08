pub struct Query;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestCommitVars {
    pub repo: String,
    pub owner: String,
    pub quantity: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentCommitVars {
    pub owner: String,
    pub quantity: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitsByTimeVars {
    pub repo: String,
    pub owner: String,
    pub branch: String,
    pub since: String,
    pub until: String,
}

impl Query {
    pub fn latest_commit_by_repo() -> String {
        String::from(
            r#"query LatestCommitsByRepo($repo: String!, $owner: String!, $quantity: Int!) {
				repository(name: $repo, owner: $owner) {
				  defaultBranchRef {
					target {
					  ... on Commit {
						history(first: $quantity) {
						  pageInfo {
							hasNextPage
							hasPreviousPage
						  }
						  edges {
							node {
							  	oid
                messageHeadline
								commitUrl
								committedDate
								changedFiles
								author {
									name
									email
									date
									avatarUrl
								}
							}
						  }
						}
					  }
					}
				  }
				}
			  }"#,
        )
    }

    pub fn recent_active_repos() -> String {
        String::from(
            r#"query RecentActiveRepos($owner: String!, $quantity: Int!) {
    			repositoryOwner(login: $owner) {
    			  repositories(
    				orderBy: {field: PUSHED_AT, direction: DESC}
    				isFork: false
    				first: $quantity
    			  ) {
    				edges {
    				  node {
    					name
    					description
    					pushedAt
						url
    				  }
    				}
    			  }
    			  avatarUrl
    			}
    		  }"#,
        )
    }
    // "2022-11-06T23:59:59Z"
    pub fn commits_in_a_day() -> String {
        String::from(
            r#"
          query CommitsInADay($repo: String!, $owner: String!, $branch: String!, $since: GitTimestamp!, $until: GitTimestamp!) {
            repository(name: $repo, owner: $owner) {
              url
              name
              description
              object(expression: $branch) {
                ... on Commit {
                  history(since: $since, until: $until) {
                    edges {
                      node {
                        author {
                          name
                        }
                        committedDate
                        changedFiles
                        messageHeadline
                        oid
                        url
                      }
                    }
                    totalCount
                  }
                }
              }
            }
          }
          "#,
        )
    }
}
