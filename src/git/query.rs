pub struct Query;

impl Query {
    pub fn query_latest_commit_by_repo() -> String {
        String::from(
            r#"query MyQuery($name: String = "git-stats-bot", $owner: String = "maemreyo", $first: Int = 10) {
				repository(name: $name, owner: $owner) {
				  defaultBranchRef {
					target {
					  ... on Commit {
						history(first: $first) {
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
