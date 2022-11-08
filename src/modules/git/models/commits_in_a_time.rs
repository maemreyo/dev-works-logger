use serde::{Deserialize, Serialize};

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Eq
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
    Serialize,
    Deserialize,
    Eq
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
    Serialize,
    Deserialize,
    Eq
)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub object: Object,
    pub url: String,
    pub description: String,
    pub name: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Eq
)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub history: History,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Eq
)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub edges: Vec<Edge>,
    pub total_count: i64,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Eq
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
    Serialize,
    Deserialize,
    Eq
)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub author: Author,
    pub changed_files: u64,
    pub message_headline: String,
    pub oid: String,
    pub url: String,
    pub committed_date: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Eq
)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
}
