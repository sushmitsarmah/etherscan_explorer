use clap::Parser;
use clap::{arg, command};

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Account(AccountArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct AccountArgs {
    #[arg(long)]
    pub account: Option<String>,

    #[arg(long)]
    pub action: Option<String>,
}