use serde::Deserialize;
use reqwest::Client;
use rust_decimal::Decimal;

pub struct WBan {
    wban_api: String,
}

#[derive(Debug, Deserialize)]
struct PendingBalance {
    amount: String,
}

impl WBan {
    pub fn new(wban_api: String) -> WBan {
        WBan {
            wban_api: wban_api
        }
    }

    pub async fn fetch_pending_withdrawals_balance(&self) -> Result<Decimal, reqwest::Error> {
        let response: PendingBalance = Client::new()
            .get(format!("{}/withdrawals/pending", self.wban_api))
            .send().await?
            .json().await?;
        Ok(Decimal::from_str_radix(response.amount.as_str(), 10).unwrap())
    }
}