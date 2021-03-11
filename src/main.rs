mod banano;
mod wban;
mod reddit;

use crate::banano::Banano;
use crate::wban::WBan;
use crate::reddit::{Notifier, RedditNotifier};
use error_chain::error_chain;
use rust_decimal::Decimal;
use std::env;

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() ->  Result<()> {
    let wban_api = env::var("WBAN_API").expect("Missing WBAN_API env variable");
    let banano_rpc_api = env::var("BAN_RPC_API").expect("Missing BAN_RPC_API env variable");
    let hot_wallet = env::var("BAN_HOT_WALLET").expect("Missing BAN_HOT_WALLET env variable");
    let cold_wallet = env::var("BAN_COLD_WALLET").expect("Missing BAN_COLD_WALLET env variable");
    let threshold_percentage = env::var("THRESHOLD_PERCENTAGE").expect("Missing THRESHOLD_PERCENTAGE env variable");

    println!("Balances:");

    let banano = Banano::new(banano_rpc_api);
    let hot_wallet_balance: Decimal = banano.get_banano_balance(&hot_wallet).await?;
    let cold_wallet_balance: Decimal = banano.get_banano_balance(&cold_wallet).await?;
    let total_users_deposits_balance: Decimal = hot_wallet_balance
        .checked_add(cold_wallet_balance)
        .unwrap();
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

    let notifier: Box<dyn Notifier> = RedditNotifier::new(vec!(String::from("wrap-that-potassium")));
    if needed_extra_balance.is_sign_positive() && !needed_extra_balance.is_zero() {
        let message = format!("I need {:#?} BAN to be sent to hot wallet \"{}\", in order to reach {:#?}% of users deposits.",
            needed_extra_balance.ceil(), &hot_wallet, percentage
        );
        println!("{}", message);
        notifier.ask_for_cold_wallet_funds(&message).await?;
    } else {
        println!("No need for more BAN!");
    }

    Ok(())
}
