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
    pub repository_owner: RepositoryOwner,
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
pub struct RepositoryOwner {
    pub repositories: Repositories,
    pub avatar_url: String,
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
pub struct Repositories {
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
    pub name: String,
    pub description: String,
    pub pushed_at: String,
    pub id: String,
    pub url: String,
}
