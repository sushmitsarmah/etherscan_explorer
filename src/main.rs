use dotenv::dotenv;

// import lib functions
use etherscan_explorer_cmd_line::init_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let etherscan_key = std::env::var("ETHERSCAN_KEY").expect("ETHERSCAN_KEY must be set.");

    let result = init_fn(etherscan_key).await?;

    Ok(result)
}
