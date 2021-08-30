use roux::{
    Reddit,
    util::error::RouxError
};
use async_trait::async_trait;
use std::env;

#[async_trait]
pub trait Notifier {
    async fn ask_for_cold_wallet_funds(&self, message: &String) -> Result<(), RouxError> ;
}

pub struct RedditNotifier {
    user_agent: String,
    users: Vec<String>,
    reddit_bot_username: String,
    reddit_bot_password: String,
    client_id: String,
    client_secret: String,
}

impl RedditNotifier {
    pub fn new(users: Vec<String>) -> Box<dyn Notifier> {
        Box::new(RedditNotifier {
            user_agent: String::from("linux:wban-notifier:0.1 by /u/wrap-that-potassium"),
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
    async fn ask_for_cold_wallet_funds(&self, message: &String) -> Result<(), RouxError> {
        let client = Reddit::new(self.user_agent.as_str(), self.client_id.as_str(), self.client_secret.as_str())
            .username(self.reddit_bot_username.as_str())
            .password(self.reddit_bot_password.as_str())
            .login()
            .await;
        let me = client.unwrap();

        // send DMs
        for username in self.users.iter() {
            println!("Sending DM to {:#?}", username);
            let resp = me.compose_message(username, "wBAN needs some BAN from the cold wallet", message).await;
            if resp.is_err() {
                panic!("Can't send Reddit DM");
            }
        }

        Ok(())
    }
}