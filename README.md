# Personal Finance CLI Manager

A fast, clean, and fully interactive command‑line tool for managing personal income, expenses, budgets, and monthly reports.  
Includes a modern TUI (Terminal User Interface) built with **ratatui**, offering a clear and intuitive financial overview.

---

## Features

### **Smart CSV Import**
- Supports CSV files with flexible date formats (`M/D/YYYY`, `MM/DD/YYYY`)
- Automatically normalizes dates to `MM/DD/YYYY`
- Validates amounts and skips invalid rows
- Automatically categorizes transactions based on the description, if category field is empty

### **Interactive TUI**
- Scrollable transaction list with color‑coded entries  
  - **Green** → income  
  - **Red** → expenses  
  - **Blue** → selected row  
- Integrated header inside the transactions block  
- Keyboard Shortcuts:
```
|  Key  | Action |
| ↑ / ↓ | Navigate transactions |
| ← / → | Switch tabs |
|   q   | Quit TUI |
```

### **Monthly Budgets**
- Set budgets per category
- Automatically calculates spending for the current month
- ASCII progress bars
- Color indicators:
  - **Green** → under 70%
  - **Yellow** → 70–100%
  - **Red** → over budget
- Automatically receive alerts when a transaction sends the amount spent over the budget limit for the current month

### **Monthly Reports**
- Income, expenses, and net balance for the current month
- Top spending categories
- ASCII bar chart for category breakdown

### **Full CLI Support**
Available commands:
- `import` – import transactions from CSV  
- `tui` – launch the interactive terminal UI  
- `add` – add a transaction manually  
- `budget` – manage budgets  
- `report` – generate quick CLI reports
- `search` – search for a specific transaction / all transactions

### **Data Storage**
- All data is stored locally in a SQLite database: "finance.db"
- This file is created automatically on first run.

---

## Installation

```bash
git clone <REPO-URL>
cd personal-finance-cli
cargo build --release
```

## Example commands
```bash
cargo run -- tui
cargo run -- import --path <FILE-PATH>
cargo run -- add --amount=<AMOUNT> --date <DATE> --category <CATEGORY> --desc <DESC>
cargo run -- search
cargo run -- search --category <CATEGORY> --month <MONTH>
cargo run -- budget set --category <CATEGORY> --limit <LIMIT>
cargo run -- budget list
cargo run -- report
```

## Accepted CSV format
```
date,amount,category,description
1/2/2026,-50.90,Food,KFC
03/01/2026,1000,Income,Salary
```

---

## TUI overview
Transactions tab with color coded entries
<img width="1098" height="477" alt="image" src="https://github.com/user-attachments/assets/2be58f4d-f246-40a3-843d-abd0f0e88b69" />

Budgets tab with progress bars 
<img width="1100" height="185" alt="image" src="https://github.com/user-attachments/assets/5d4b1795-e5ba-412e-8dc4-4cddf8fb2dd1" />

Monthly reports with ASCII charts 
<img width="1092" height="372" alt="image" src="https://github.com/user-attachments/assets/7ba774b9-15fc-46a2-9677-9450e2511690" />

