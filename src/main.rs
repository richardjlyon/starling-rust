mod cli;

use clap::Parser;

use crate::cli::commands::initialise::initialise;
use crate::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { start_date } => initialise(start_date).await?,
    }

    Ok(())
}
