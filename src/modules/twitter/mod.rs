#![allow(unused)]
pub mod common;

pub struct Twitter;
use anyhow::Ok;
use egg_mode::tweet::DraftTweet;
use log::info;
use tokio::sync::watch::error;

impl Twitter {
    pub async fn new_tweet(content: String) -> anyhow::Result<()> {
        let config = common::Config::load().await;
        let mut tweet = DraftTweet::new(content);
        let response = tweet.send(&config.token).await?;
        Ok(())
    }
}
