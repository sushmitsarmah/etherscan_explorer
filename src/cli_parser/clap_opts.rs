use clap::Parser;
use clap::{arg, command};

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Balance(BalanceArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct BalanceArgs {
    #[arg(long)]
    pub account: Option<String>,
}