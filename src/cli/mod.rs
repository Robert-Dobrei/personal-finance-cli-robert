use clap::{Parser, Subcommand};

#[derive(Parser)]
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
        category: Option<String>,
        #[arg(long)]
        date: String,
        #[arg(long, default_value = "")]
        desc: String,
    },

    Search {
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        month: Option<String>,
    },

    Import {
        #[arg(long)]
        path: String,
    },

    Budget {
        #[command(subcommand)]
        action: BudgetAction,
    },
}

#[derive(Subcommand)]
pub enum BudgetAction {
    Set {
        #[arg(long)]
        category: String,
        #[arg(long)]
        limit: f64,
    },
    List,
}
