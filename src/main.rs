mod banano;
mod wban;
mod notifiers;

use crate::banano::Banano;
use crate::wban::WBan;
use crate::notifiers::{Notifier, TelegramNotifier};
use rust_decimal::Decimal;
use dotenv::dotenv;
use std::env;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() ->  Result<()> {
    dotenv().ok(); // Read .env and set env variables with this

    let wban_api = env::var("WBAN_API").expect("Missing WBAN_API env variable");
    let blockchain_network = env::var("BLOCKCHAIN_NETWORK").unwrap_or(String::from("BSC"));
    let banano_rpc_api = env::var("BAN_RPC_API").expect("Missing BAN_RPC_API env variable");
    let hot_wallet = env::var("BAN_HOT_WALLET").expect("Missing BAN_HOT_WALLET env variable");
    let cold_wallet = env::var("BAN_COLD_WALLET").expect("Missing BAN_COLD_WALLET env variable");
    let threshold_percentage = env::var("THRESHOLD_PERCENTAGE").expect("Missing THRESHOLD_PERCENTAGE env variable");
    /*
    let users: Vec<String> = env::var("REDDIT_BOT_DM_USERS").expect("Missing REDDIT_BOT_DM_USERS env variable")
        .split_whitespace()
        .map(|user| String::from(user))
        .collect();
    */

    println!("Balances:");

    let banano = Banano::new(banano_rpc_api);
    let hot_wallet_balance: Decimal = banano.get_banano_balance(&hot_wallet).await?;
    let cold_wallet_balance: Decimal = banano.get_banano_balance_with_pending(&cold_wallet).await?;
    let total_users_deposits_balance: Decimal = hot_wallet_balance
        .checked_add(cold_wallet_balance)
        .context("Overflow when adding hot and cold BAN balances")?;
    println!("\tHot wallet:\t\t{:#?} BAN", hot_wallet_balance);
    println!("\tCold wallet:\t\t{:#?} BAN", cold_wallet_balance);
    println!("\t\t\t\t----------------------");
    println!("\tTotal users deposits:\t{:#?} BAN\n", total_users_deposits_balance);

    let wban = WBan::new(wban_api);
    let pending_withdrawals_balance: Decimal = wban.fetch_pending_withdrawals_balance().await?;
    println!("Pending withdrawals: {:#?} BAN\n", pending_withdrawals_balance);

    let percentage: Decimal = Decimal::from_str_radix(threshold_percentage.as_str(), 10).unwrap();
    let threshold: Decimal = percentage.checked_div(Decimal::from(100)).unwrap();

    // needed = (total - pending) * threshold + pending - hot
    let needed_extra_balance: Decimal = total_users_deposits_balance
        .checked_sub(pending_withdrawals_balance).unwrap()
        .checked_mul(threshold).unwrap()
        .checked_add(pending_withdrawals_balance).unwrap()
        .checked_sub(hot_wallet_balance).unwrap();

    if needed_extra_balance.is_sign_positive() && !needed_extra_balance.is_zero() {
        let message = format!("I need `{:#?}` BAN to be sent to *{}* hot wallet `{}`, in order to reach {:#?}% of users deposits",
            needed_extra_balance.ceil(), &blockchain_network, &hot_wallet, percentage
        );
        println!("{}", message);
        let notifier: Box<dyn Notifier> = TelegramNotifier::new();
        notifier.ask_for_cold_wallet_funds(&message).await.unwrap();
    } else {
        println!("No need for more BAN!");
    }

    Ok(())
}
