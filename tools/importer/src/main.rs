use clap::{Parser, Subcommand};

use crate::logger::debug;

mod commands;
mod logger;

#[macro_export]
macro_rules! ternary {
    ($condition: expr => $true_expr: expr , $false_expr: expr) => {
        if $condition { $true_expr } else { $false_expr }
    };
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    api_key: String,
    #[arg(short, long)]
    base_id: String,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Guestlog {},
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();
    debug("Debug logging enabled", cli.verbose);
    match &cli.command {
        Commands::Guestlog {} => {
            commands::guestlog::guestlog(cli.verbose, cli.api_key, cli.base_id)
                .await
                .unwrap();
        }
    }
}
