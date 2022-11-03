pub struct Query;

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
								message
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
}
