use reqwest;
use serde::Deserialize;
use web3_unit_converter::Unit;
use serde_json;

const API: &str = "https://api.etherscan.io/api";
const MODULE: &str = "account";

#[derive(serde::Deserialize, Debug)]
struct Account {
    message: String,
    result: String,
}

#[derive(Deserialize, Debug)]
pub struct AccTxn {
    #[serde(rename = "blockNumber")]
    block_number: String,

    #[serde(rename = "timeStamp")]
    time_stamp: String,

    hash: String,
    nonce: String,
    // #[serde(rename = "blockHash")]
    // block_hash: String,

    #[serde(rename = "transactionIndex")]
    transaction_index: String,

    from: String,
    to: String,
    value: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,

    #[serde(rename = "isError")]
    is_error: String,

    txreceipt_status: String,
    input: String,
    #[serde(rename = "contractAddress")]
    contract_address: String,

    #[serde(rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,

    #[serde(rename = "gasUsed")]
    gas_used: String,

    confirmations: String,
    #[serde(rename = "methodId")]
    method_id: String,

    #[serde(rename = "functionName")]
    function_name: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountTxns {
    message: String,
    result: Vec<AccTxn>, // AccTxn,
    // result: Vec<AccTxn>, // AccTxn,
}

pub async fn get_account_balance(
    address : &str,
    etherscan_key: String
) -> Result<String, Box<dyn std::error::Error>> {

    let action = "balance";
    let tag = "latest";

    let url = API.to_string()
        + "?module=" + MODULE
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

pub async fn get_account_txns(
    address : &str,
    etherscan_key: String
) -> Result<Vec<AccTxn>, Box<dyn std::error::Error>> {

    let action = "txlist";
    let start_block = "0";
    let end_block = "99999999";
    let page = "1";
    let offset = "10";
    let sort = "asc";

    let url = API.to_string()
        + "?module=" + MODULE
        + "&action=" + action
        + "&address=" + address
        + "&startblock=" + start_block
        + "&endblock=" + end_block
        + "&page=" + page
        + "&offset=" + offset
        + "&sort=" + sort;

    let apikey = format!("&apikey={}", etherscan_key.as_str());
    let url = url + apikey.as_str();

    let body = reqwest::get(url).await?
        .text().await?;

    let dbody: AccountTxns = serde_json::from_str(&body)?;
    let result = dbody.result;

    return Ok(result)
}