use crate::{
    slack_api::{conversations::Conversations, users::Users},
    SLACK_CLIENT,
};
use actix_web::{http::StatusCode, post, HttpResponse, ResponseError};
use actix_web::{web, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormParams {
    #[serde(rename = "channel_id")]
    channel_id: String,
}

#[post("/")]
async fn handle_invite(req: web::Form<FormParams>) -> Result<HttpResponse> {
    // 3s以内にレスポンスを返さないといけないので別スレッドで処理を行う
    tokio::spawn(async {
        let req = req;
        let channel_id = &req.channel_id;
        let users_list = SLACK_CLIENT.get_users_list().await?;
        let mut users_set: HashSet<&str> =
            HashSet::from_iter(users_list.iter().map(|user| user.id.as_str()));

        let members = SLACK_CLIENT.get_conversations_members(channel_id).await?;

        for member in &members {
            users_set.remove(member.as_str());
        }

        SLACK_CLIENT
            .invite(
                channel_id,
                &users_set.into_iter().collect::<Vec<&str>>()[..],
            )
            .await?;

        Ok::<(), anyhow::Error>(())
    });

    Ok(HttpResponse::Ok().body(""))
}

#[derive(Debug)]
pub struct ApiErrorResponse {
    pub status: StatusCode,
    pub message: String,
}

impl ApiErrorResponse {
    #[allow(dead_code)]
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}

impl actix_web::error::ResponseError for ApiErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.status
    }
}

impl Display for ApiErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.status_code().as_str(), self.message)
    }
}
