use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Finance CLI")]
#[command(version = "0.1")]
#[command(about = "Track income and expenses")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        category: String,
        #[arg(long)]
        date: String,
        #[arg(long, default_value = "")]
        desc: String,
    },
    Report,
    Budget,
    Import {
        #[arg(long)]
        path: String,
    },
    Search {
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        month: Option<String>,
    },
}
