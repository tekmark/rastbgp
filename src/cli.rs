use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rastbgp")]
#[command(about = "A fast Rust BGP daemon", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(short, long)]
        config: Option<String>,
    },
    Status,
    Rib,
    Metrics {
        #[arg(short, long, default_value_t = 9898)]
        port: u16,
    },
}