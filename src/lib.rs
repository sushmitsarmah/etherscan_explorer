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
use cli_parser::display_table::{display_table_new, TableRow};
use etherscan::account::AccTxn;

pub async fn fetch_eth_txns_data(
    address: String,
    etherscan_key:String
) -> Result<Vec<AccTxn>, Box<dyn std::error::Error>> {
    let txns = etherscan::account::get_account_txns(
        address.as_str(),
        etherscan_key
    ).await?;

    Ok(txns)
}

pub async fn init_fn(etherscan_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let CargoCli::Account(args) = CargoCli::parse();

    if let Some(address) = args.account {
        if let Some(action) = args.action {
            if action == "txns" {
                let txns = fetch_eth_txns_data(address, etherscan_key).await?;

                let rows: Vec<TableRow> = txns
                    .iter()
                    .map(|txn| {
                        TableRow::new(
                            txn.block_number.to_string(),
                            txn.time_stamp.to_string(),
                            txn.from.to_string(),
                            txn.to.to_string(),
                            txn.value.to_string(),
                        )
                    })
                    .collect();

                display_table_new(rows);
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
