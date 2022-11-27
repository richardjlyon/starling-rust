//! A command line utility for budget
//!
//!

use clap::{Parser, Subcommand};
pub mod commands;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialises a beancount file
    Init {
        /// Start date (YYYY-MM-DD)
        start_date: Option<String>,
    },
}
