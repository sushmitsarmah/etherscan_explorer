use serde_json;
use reqwest;
use dotenv::dotenv;

use std::collections::HashMap;
use web3_unit_converter::Unit;

mod etherscan;

const API: &str = "https://api.etherscan.io/api";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let etherscan_key = std::env::var("ETHERSCAN_KEY")
        .expect("ETHERSCAN_KEY must be set.");

    let module = "module";
    let address = "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae";
    let action = "balance";
    let tag = "latest";

    let url = API.to_string()
        + "?module=" + module
        + "&action=" + action
        + "&address=" + address
        + "&tag=" + tag;

    let apikey = format!("&apikey={}", etherscan_key.as_str());

    let url = url + apikey.as_str();

    println!("{:#?}", url);

    let body = reqwest::get(url).await?
        .text().await?;

    let dbody: HashMap<String, String> = serde_json::from_str(&body)?;
    // let wei = dbody.get("result");

    if let Some(wei) = dbody.get("result") {
        let one_wei_in_eth = Unit::Wei(&wei.to_string()).to_eth_str().unwrap();
        println!("{:#?}", one_wei_in_eth);
    }

    // println!("Hello, world! {}", etherscan_key);
    Ok(())
}
