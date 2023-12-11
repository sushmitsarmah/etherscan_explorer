use clap::Parser;
use dotenv::dotenv;

mod cli_parser;
mod etherscan;

use cli_parser::clap_opts::CargoCli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let etherscan_key = std::env::var("ETHERSCAN_KEY").expect("ETHERSCAN_KEY must be set.");

    let CargoCli::Account(args) = CargoCli::parse();

    if let Some(address) = args.account {
        if let Some(action) = args.action {
            if action == "txns" {
                let txns = etherscan::account::get_account_txns(
                    address.as_str(),
                    etherscan_key
                ).await?;
                // println!("{:#?}", txns);
                cli_parser::display_table::display_table(txns);
            } else {
                let balance =
                    etherscan::account::get_account_balance(address.as_str(), etherscan_key)
                        .await?;

                println!("Account: {}", address);
                println!("Balance: {}", balance);
            }
        }
    } else {
        println!("No Account provided!!");
    }

    Ok(())
}
