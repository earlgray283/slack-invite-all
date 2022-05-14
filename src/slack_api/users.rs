use super::{
    models::users::{self, UsersListResponse},
    Client,
};
use anyhow::{bail, Ok, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Users {
    async fn get_users_list(&self) -> Result<Vec<users::Member>>;
}

#[async_trait]
impl Users for Client {
    async fn get_users_list(&self) -> Result<Vec<users::Member>> {
        let req = self
            .http
            .get("https://slack.com/api/users.list")
            .bearer_auth(&self.oauth_token);
        let api_resp = req.send().await?.json::<UsersListResponse>().await?;
        if let Some(e) = api_resp.error {
            bail!("GET https://slack.com/api/users.list: {}", e);
        }
        Ok(api_resp.members.unwrap())
    }
}
