use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Terminal,
};
use rusqlite::Connection;
use crate::db::Transaction;
use crate::reports::ReportSummary;
use chrono::Local;
use chrono::Datelike;

pub struct App {
    pub tabs: Vec<&'static str>,
    pub index: usize,
    pub last_input: Instant,
    pub transactions: Vec<Transaction>,
    pub selected_tx: usize,
	pub budgets: Vec<(String, f64, f64)>, 
}

impl App {
    fn new(conn: &Connection) -> Self {
		
		let transactions = load_transactions(conn);
		let budgets = load_budgets(conn);

		Self {
			tabs: vec!["Transactions", "Budgets", "Reports"],
			index: 0,
			last_input: Instant::now(),
			transactions,
			selected_tx: 0,
			budgets,
		}
	}
    
    fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    fn previous_tab(&mut self) {
        if self.index == 0 {
            self.index = self.tabs.len() - 1;
        } else {
            self.index -= 1;
        }
    }

    fn can_accept_input(&mut self) -> bool {
        let delay = Duration::from_millis(150);
        if self.last_input.elapsed() >= delay {
            self.last_input = Instant::now();
            true
        } else {
            false
        }
    }

    fn next_tx(&mut self) {
        if self.selected_tx + 1 < self.transactions.len() {
            self.selected_tx += 1;
        }
    }

    fn previous_tx(&mut self) {
        if self.selected_tx > 0 {
            self.selected_tx -= 1;
        }
    }
}

fn load_transactions(conn: &Connection) -> Vec<Transaction> {
    match crate::db::search_transactions(conn, None, None) {
        Ok(list) => list,
        Err(_) => vec![],
    }
}

pub fn run_tui(conn: Connection) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, conn);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    conn: Connection,
) -> io::Result<()> {
    let mut app = App::new(&conn);
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_tx));

    loop {
        list_state.select(Some(app.selected_tx + 1));

        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let titles: Vec<Span> = app
                .tabs
                .iter()
                .map(|t| Span::styled(*t, Style::default().fg(Color::Yellow)))
                .collect();
            let tabs = Tabs::new(titles)
                .select(app.index)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Cyan));
            f.render_widget(tabs, chunks[0]);

            match app.index {
                0 => {

                    let list = render_transactions(&app);
                    f.render_stateful_widget(list, chunks[1], &mut list_state);
                }
                1 => {
                    let list = render_budgets(&app);
					f.render_widget(list, chunks[1]);
                }
                2 => {
                    let now = Local::now();
					let year = now.year();
					let month = now.month();

					let summary = crate::reports::generate_monthly_summary(&conn, year, month);
					let cat_data = crate::reports::expenses_by_category_for_month(&conn, year, month);

					let ascii_chart = ascii_barchart(&cat_data);
					let text = format!(
						"{}\n\nExpenses by Category:\n{}",
						render_reports_text(&summary),
						ascii_chart
					);
					let current_month = now.format("%B %Y").to_string();
					let title = format!("Reports for {}", current_month);

					let widget = Paragraph::new(text)
						.block(Block::default().title(title).borders(Borders::ALL));
					f.render_widget(widget, chunks[1]);
                }
                _ => {}
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        if app.can_accept_input() {
                            app.previous_tab();
                        }
                    }
                    KeyCode::Right => {
                        if app.can_accept_input() {
                            app.next_tab();
                        }
                    }
                    KeyCode::Up => {
                        if app.index == 0 && app.can_accept_input() {
                            app.previous_tx();
                        }
                    }
                    KeyCode::Down => {
                        if app.index == 0 && app.can_accept_input() {
                            app.next_tx();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn render_transactions(app: &App) -> List<'static> {
    let mut items: Vec<ListItem> = Vec::new();

    let header = format!(
        "{:<10} {:>10}  {:<12}  {}",
        "Date", "Amount", "Category", "Description"
    );

    items.push(
        ListItem::new(header)
            .style(Style::default().fg(Color::Yellow))
    );

    for tx in &app.transactions {
        let amount_color = if tx.amount < 0.0 {
            Color::Red
        } else {
            Color::Green
        };

        let line = format!(
            "{:<10} {:>10.2}  {:<12}  {}",
            tx.date,
            tx.amount,
            tx.category,
            tx.description
        );

        items.push(
            ListItem::new(line)
                .style(Style::default().fg(amount_color))
        );
    }

    List::new(items)
        .block(Block::default().title("Transactions").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
}


fn render_budgets(app: &App) -> List<'static> {
    let items: Vec<ListItem> = app.budgets.iter().map(|(cat, limit, spent)| {
        let pct = if *limit > 0.0 { spent / limit } else { 0.0 };

        let color = if pct < 0.7 {
            Color::Green
        } else if pct < 1.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        let bar_len = (pct * 20.0).min(20.0) as usize;
        let bar = format!(
            "[{}{}]",
            "#".repeat(bar_len),
            ".".repeat(20 - bar_len)
        );

        let line = format!(
            "{:<12}  {:>6.0}/{:<6.0}  {}",
            cat,
            spent,
            limit,
            bar
        );

        ListItem::new(line).style(Style::default().fg(color))
    }).collect();

    let current_month = chrono::Local::now().format("%B %Y").to_string();
    let title = format!("Budgets for {}", current_month);

    List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
}

fn load_budgets(conn: &Connection) -> Vec<(String, f64, f64)> {
    let mut result = Vec::new();

    if let Ok(budgets) = crate::budget::list_budgets(conn) {
        for (category, limit, _spent) in budgets {
            let month = chrono::Local::now().format("%Y-%m").to_string();
            let spent = crate::budget::get_monthly_spent(conn, &category, &month)
                .unwrap_or(0.0)
                .abs();

            result.push((category, limit, spent));
        }
    }

    result
}

fn ascii_barchart(data: &[(String, f64)]) -> String {
    let mut out = String::new();

    let max = data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    let scale = if max > 0.0 { 20.0 / max } else { 1.0 };

    for (cat, val) in data {
        let bars = (val * scale).round() as usize;
        out.push_str(&format!("{:<12} {:<20} {:.2}\n",
            cat,
            "█".repeat(bars),
            val
        ));
    }

    out
}

fn render_reports_text(summary: &ReportSummary) -> String {
    let mut s = String::new();

    s.push_str(&format!("Total income: {:.2}\n", summary.total_income));
    s.push_str(&format!("Total expenses: {:.2}\n", summary.total_expenses));
    s.push_str(&format!("Net: {:.2}\n\n", summary.net));

    s.push_str("Top categories:\n");
    let mut cats: Vec<_> = summary.by_category.iter().collect();
    cats.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (cat, amt) in cats.into_iter().take(3) {
        s.push_str(&format!(" - {}: {:.2}\n", cat, amt));
    }

    s
}