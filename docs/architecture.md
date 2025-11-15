# Architecture - Personal Finance CLI Manager

## Overview
This project is a command-line tool for managing personal finances.  
It allows importing transactions from CSV/OFX files, manual entry of income/expenses, automatic categorization, budget tracking, and generating reports directly in the terminal.

## Components
- **CLI Layer**: Handles subcommands (`add`, `import`, `report`, `budget`, `search`).
- **Parser**: Reads CSV/OFX formats and converts them into transaction objects.
- **Database Layer**: SQLite database for persistent storage.
- **Categorization Engine**: Applies regex-based rules to assign categories.
- **Reporting Module**: Generates monthly summaries, category breakdowns, and budget alerts. Future extension: interactive TUI.

## Data Flow
1. User runs a CLI command.
2. Input is parsed (CSV/OFX or manual).
3. Transactions are categorized.
4. Data is stored in SQLite.
5. Reports are generated and displayed in terminal.

## Repository Structure
See project tree
