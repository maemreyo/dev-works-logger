use serde::{Deserialize, Serialize};

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub repository: Repository,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub default_branch_ref: DefaultBranchRef,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBranchRef {
    pub target: Target,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub history: History,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub oid: String,
    pub message: String,
    pub author: Author,
    pub commit_url: String,
    pub committed_date: String,
    pub changed_files: i64,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: String,
    pub avatar_url: String,
}
