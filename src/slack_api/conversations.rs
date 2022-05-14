use super::{
    models::conversations::{Channel, ConversationsResponse, Error},
    Client,
};
use anyhow::{bail, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Conversations {
    async fn invite(&self, channel: &str, user_id: &[&str]) -> Result<Channel>;
    async fn get_conversations_members(&self, channel: &str) -> Result<Vec<String>>;
}

#[async_trait]
impl Conversations for Client {
    async fn invite(&self, channel: &str, user_id: &[&str]) -> Result<Channel> {
        let req = self
            .http
            .post("https://slack.com/api/conversations.invite")
            .bearer_auth(&self.oauth_token)
            .query(&[("channel", channel), ("users", user_id.join(",").as_str())]);
        let api_resp = req.send().await?.json::<ConversationsResponse>().await?;
        if let Some(e) = api_resp.error {
            dbg!(&e);
            bail!(Error::from(&e));
        }
        Ok(api_resp.channel.unwrap())
    }

    async fn get_conversations_members(&self, channel: &str) -> Result<Vec<String>> {
        let req = self
            .http
            .post("https://slack.com/api/conversations.members")
            .bearer_auth(&self.oauth_token)
            .query(&[("channel", channel)]);
        let api_resp = req.send().await?.json::<ConversationsResponse>().await?;
        if let Some(e) = api_resp.error {
            dbg!(&e);
            bail!(Error::from(&e));
        }
        Ok(api_resp.members.unwrap())
    }
}
