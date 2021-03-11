use reqwest::Client;
use serde::Deserialize;
use async_trait::async_trait;
use std::env;

#[async_trait]
pub trait Notifier {
    async fn ask_for_cold_wallet_funds(&self, message: &String) -> Result<(), reqwest::Error> ;
}

pub struct RedditNotifier {
    user_agent: String,
    users: Vec<String>,
    reddit_bot_username: String,
    reddit_bot_password: String,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Deserialize)]
struct RedditOAuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

impl RedditNotifier {
    pub fn new(users: Vec<String>) -> Box<dyn Notifier> {
        Box::new(RedditNotifier {
            user_agent: String::from("server:wBAN-Notifier v0.1 by /u/jeromebernard"),
            users: users,
            reddit_bot_username: env::var("REDDIT_BOT_USERNAME").expect("Missing REDDIT_BOT_USERNAME env variable"),
            reddit_bot_password: env::var("REDDIT_BOT_PASSWORD").expect("Missing REDDIT_BOT_PASSWORD env variable"),
            client_id: env::var("REDDIT_BOT_CLIENT_ID").expect("Missing REDDIT_BOT_CLIENT_ID env variable"),
            client_secret: env::var("REDDIT_BOT_CLIENT_SECRET").expect("Missing REDDIT_BOT_CLIENT_SECRET env variable"),
        })
    }
}

#[async_trait]
impl Notifier for RedditNotifier {
    async fn ask_for_cold_wallet_funds(&self, message: &String) -> Result<(), reqwest::Error> {
        // request OAuth2 token
        let params = [
            ("grant_type", "password"),
            ("username", self.reddit_bot_username.as_str()),
            ("password", self.reddit_bot_password.as_str()),
        ];
        let response: RedditOAuthTokenResponse = Client::new()
            .post("https://www.reddit.com/api/v1/access_token")
            .header("User-Agent", self.user_agent.clone())
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str()))
            .form(&params)
            .send().await?
            .json().await?;
        let access_token = response.access_token;

        // send DMs
        for username in self.users.iter() {
            self.send_dm(&access_token, username, &message).await?;
        }

        Ok(())
    }
}

impl RedditNotifier {
    async fn send_dm(&self, access_token: &String, username: &String, message: &String) -> Result<(), reqwest::Error> {
        let params = [
            ("api_type", "json"),
            ("from_sr", ""),
            ("g-recaptcha-response", ""),
            ("subject", "wBAN needs some BAN from the cold wallet"),
            ("text", message),
            ("to", username.as_str()),
            ("uh", "")
        ];
        let _response = Client::new()
            .post("https://oauth.reddit.com/api/compose")
            .header("User-Agent", self.user_agent.clone())
            .bearer_auth(access_token)
            .form(&params)
            .send().await?
            .text().await?;
        // println!("{:#?}", response);

        Ok(())
    }
}