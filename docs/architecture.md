# Architecture - Personal Finance CLI Manager

## Overview
This project is a command-line tool for managing personal finances.  
It allows importing transactions from CSV/OFX files, manual entry of income/expenses, automatic categorization, budget tracking, and generating insightful reports directly in the terminal.  

## Components
- **CLI Layer**: Handles subcommands (`add`, `import`, `report`, `budget`, `search`). Provides a consistent interface for all user actions.
- **Parser**: Reads CSV/OFX formats and converts them into transaction objects. Ensures compatibility with common bank statement formats.
- **Database Layer**: SQLite database for persistent local storage. Contains tables for transactions, budgets, and alerts.
- **Categorization Engine**: Applies regex-based rules to automatically assign categories (e.g., Food, Transport, Entertainment).
- **Budget Module**: Allows users to set spending limits per category. Stores budgets in SQLite, checks transactions against limits, and generates alerts when exceeded.
- **Search & Filter Module**: Provides commands to query transactions by date, category, amount, or description.
- **Reporting Module**: Generates monthly summaries, category breakdowns, and budget alerts.

## Data Flow
1. User runs a CLI command.
2. Input is parsed (CSV/OFX or manual entry).
3. Transactions are categorized using regex rules.
4. Data is stored in SQLite (`transactions` table).
5. Budgets are checked and alerts generated if limits are exceeded (`budgets` + `alerts` tables).
6. User can search/filter transactions.
7. Reports and charts are generated and displayed in terminal/TUI.

## Repository Structure
personal-finance-cli-robert/
├── src/              # source code
│   ├── cli/          # CLI commands
│   ├── parser/       # CSV/OFX parsing
│   ├── db/           # SQLite interactions
│   ├── categorize/   # categorization engine
│   ├── budget/       # budget tracking
│   ├── report/       # reporting & charts
│   └── search/       # search & filter
├── tests/            # unit and integration tests
├── docs/             # documentation
│   ├── screenshots/
│   ├── architecture.md
│   └── decisions.md
├── data/             # sample CSV/OFX files
├── Cargo.toml        # Rust dependencies
├── Cargo.lock        
└── README.md         # project overview
