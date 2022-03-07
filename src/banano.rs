use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use rust_decimal::Decimal;
use thiserror::Error;

pub struct Banano {
    rpc_api_host: String,
}

#[derive(Error, Debug)]
pub enum BananoError {
    #[error("HTTP request error")]
    BananoApiError(#[from] reqwest::Error),
    #[error("Overflow error")]
    Overflow,
    #[error("Decimal error")]
    Decimal(#[from] rust_decimal::Error),
}

#[derive(Debug, Deserialize)]
struct Balance {
    balance: String,
    pending: String,
}

impl Banano {

    pub fn new(banano_rpc_api_host: String) -> Banano {
        Banano {
            rpc_api_host: banano_rpc_api_host,
        }
    }

    pub async fn get_banano_balance(&self, wallet: &String) -> Result<Decimal, BananoError> {
        let balance_request = json!({
            "action": "account_balance",
            "account": wallet
        });

        let response: Balance = Client::new()
            .post(format!("http://{}", self.rpc_api_host))
            .json(&balance_request)
            .send().await?
            .json().await?;

        let balance: Decimal = self.convert_raw_balance(response.balance.clone());
        let pending: Decimal = self.convert_raw_balance(response.pending.clone());
        let total: Decimal = balance.checked_add(pending).unwrap();
        Ok(total)
    }

    pub async fn get_banano_balance_with_pending(&self, wallet: &String) -> Result<Decimal, BananoError> {
        let balance_request = json!({
            "action": "account_balance",
            "account": wallet
        });

        let response: Balance = Client::new()
            .post(format!("http://{}", self.rpc_api_host))
            .json(&balance_request)
            .send().await?
            .json().await?;

        let balance: Decimal = self.convert_raw_balance(response.balance.clone());
        let pending: Decimal = self.convert_raw_balance(response.pending.clone());
        let total: Option<Decimal> = balance.checked_add(pending);
        match total {
            Some(value) => Ok(value),
            None => Err(BananoError::Overflow),
        }
    }

    fn convert_raw_balance(&self, raw_balance: String) -> Decimal {
        if raw_balance == "0" {
            return Decimal::from(0)
        }
        
        let mut balance: String = raw_balance.clone();
        balance.truncate(balance.len() - 11);

        let mut balance: Decimal = Decimal::from_str_radix(balance.as_str(), 10).unwrap();
        balance.set_scale(18).unwrap();

        balance
    }

}
