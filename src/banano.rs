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

        let mut raw_balance: String = response.balance.clone();
        Ok(self.convert_raw_balance(&mut raw_balance))
    }

    fn convert_raw_balance(&self, raw_balance: &mut String) -> Decimal {
        if raw_balance == "0" {
            return Decimal::from(0)
        }
        
        raw_balance.truncate(raw_balance.len() - 11);

        let mut balance: Decimal = Decimal::from_str_radix(raw_balance.as_str(), 10).unwrap();
        balance.set_scale(18).unwrap();

        balance
    }

}
