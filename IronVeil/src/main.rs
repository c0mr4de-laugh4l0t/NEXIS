use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use tui::Terminal;

struct App<'a> {
    tabs: Vec<&'a str>,
    sidebar: Vec<&'a str>,
    current_tab: usize,
    selected_sidebar: usize,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            tabs: vec!["Dashboard", "Tools", "Settings"],
            sidebar: vec!["Home", "Passwords", "IP", "MAC", "VPN"],
            current_tab: 0,
            selected_sidebar: 0,
        }
    }

    fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % self.tabs.len();
    }

    fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = self.tabs.len() - 1;
        }
    }

    fn next_sidebar(&mut self) {
        self.selected_sidebar = (self.selected_sidebar + 1) % self.sidebar.len();
    }

    fn previous_sidebar(&mut self) {
        if self.selected_sidebar > 0 {
            self.selected_sidebar -= 1;
        } else {
            self.selected_sidebar = self.sidebar.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => app.previous_tab(),
                    KeyCode::Right => app.next_tab(),
                    KeyCode::Up => app.previous_sidebar(),
                    KeyCode::Down => app.next_sidebar(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Length(20), // Sidebar width
            Constraint::Min(50),    // Main content
        ])
        .split(f.size());

    let sidebar_items: Vec<ListItem> = app
        .sidebar
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_sidebar {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(*item, style))
        })
        .collect();

    let sidebar =
        List::new(sidebar_items).block(Block::default().title("Sidebar").borders(Borders::ALL));
    f.render_widget(sidebar, chunks[0]);

    let tab_titles: Vec<Spans> = app
        .tabs
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Cyan))))
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .select(app.current_tab)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(5),    // Content
        ])
        .split(chunks[1]);

    f.render_widget(tabs, layout[0]);

    let content = match app.current_tab {
        0 => format!(
            "Dashboard page — selected: {}",
            app.sidebar[app.selected_sidebar]
        ),
        1 => format!(
            "Tools page — selected: {}",
            app.sidebar[app.selected_sidebar]
        ),
        2 => format!(
            "Settings page — selected: {}",
            app.sidebar[app.selected_sidebar]
        ),
        _ => "Unknown tab".to_string(),
    };

    let paragraph =
        Paragraph::new(content).block(Block::default().title("Content").borders(Borders::ALL));
    f.render_widget(paragraph, layout[1]);
}
