use clap::{Command};

fn main() {
    let matches = Command::new("Finance CLI")
        .version("0.1")
        .author("Robert")
        .about("Track income and expenses")
        .subcommand(Command::new("add")
            .about("Add a transaction"))
        .subcommand(Command::new("report")
            .about("Generate a report"))
        .subcommand(Command::new("budget")
            .about("Manage budgets"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => println!("Add transaction (demo)"),
        Some(("report", _)) => println!("Report (demo)"),
        Some(("budget", _)) => println!("Budget (demo)"),
        _ => println!("No valid subcommand was used"),
    }
}
