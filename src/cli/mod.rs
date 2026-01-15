use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the builder on the project
    Build {
        /// Path to the project
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Serve the web project (runs index.min.json)
    Serve {
        /// Path to the project
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Port to serve on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}