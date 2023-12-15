// Usage:
// Get Account Txns: cargo run account --account <account_address> --action txns
// Get Account Balance: cargo run account --account <account_address> --action balance

// import libraries
use clap::Parser;

// initialize modules
mod cli_parser;
mod etherscan;

// use module function to get arguments from cli
use cli_parser::clap_opts::CargoCli;

pub async fn init_fn(etherscan_key: String) -> Result<(), Box<dyn std::error::Error>> {

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

