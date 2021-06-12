use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use rust_decimal::Decimal;

pub struct Banano {
    rpc_api_host: String,
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

    pub async fn get_banano_balance(&self, wallet: &String) -> Result<Decimal, reqwest::Error> {
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
