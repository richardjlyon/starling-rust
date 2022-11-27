//! TODO:
//! - serde
//! - itertools https://crates.io/crates/itertools
//! - iterator API https://doc.rust-lang.org/stable/std/iter/
//! - what is the difference between iter and into_iter
//! - thiserror
//! - tokio process https://docs.rs/tokio/latest/tokio/process/index.html
//!
//! BONUS:
//! - rayon https://crates.io/crates/rayon
//! - tokio tasks https://tokio.rs/tokio/tutorial/spawning
//! - tokio channels https://tokio.rs/tokio/tutorial/channels
//! - tokio streams https://tokio.rs/tokio/tutorial/streams

mod cli;

use clap::Parser;

use crate::cli::commands::initialise::initialise;
use crate::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            start_date,
        } => initialise(start_date).await?,
    }

    Ok(())
}
