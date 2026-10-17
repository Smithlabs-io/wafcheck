use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wafcheck")]
#[command(about = "The CI/CD safety layer for Cloudflare WAF")]


pub struct Input {
    #[command(subcommand)]
   pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {

    /// Check for configuration drift
    Drift{
        #[arg(short = 'f', long = "file", help = "Path to the file")]
        file_path: String,
    },

    /// This doc comment becomes the help text for the subcommand!
    TestKey {
    },
}

