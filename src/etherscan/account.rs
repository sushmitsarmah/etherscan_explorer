use reqwest;
use web3_unit_converter::Unit;
use serde_json;

const API: &str = "https://api.etherscan.io/api";

#[derive(serde::Deserialize, Debug)]
struct Account {
    message: String,
    result: String,
}

pub async fn get_account_balance(
    address : &str,
    etherscan_key: String
) -> Result<String, Box<dyn std::error::Error>> {

    let module = "account";
    let action = "balance";
    let tag = "latest";

    let url = API.to_string()
        + "?module=" + module
        + "&action=" + action
        + "&address=" + address
        + "&tag=" + tag;

    let apikey = format!("&apikey={}", etherscan_key.as_str());
    let url = url + apikey.as_str();
    // println!("{:#?}", url);

    let body = reqwest::get(url).await?
        .text().await?;

    let dbody: Account = serde_json::from_str(&body)?;
    // println!("{:#?}", dbody);

    if dbody.message == "NOTOK" {
        let err = dbody.result;
        return Err(err.to_owned().into())
    }

    let wei = dbody.result;
    let val_eth = Unit::Wei(&wei.to_string()).to_eth_str().unwrap();
    return Ok(val_eth)
}