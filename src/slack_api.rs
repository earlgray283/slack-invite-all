pub mod conversations;
pub mod models;
pub mod users;

pub struct Client {
    http: reqwest::Client,
    oauth_token: String,
}

impl Client {
    pub fn new(oauth_token: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            oauth_token,
        }
    }
}
