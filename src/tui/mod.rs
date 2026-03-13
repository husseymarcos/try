use crate::context::RunContext;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, Stdout};
use std::path::{Path, PathBuf};

pub fn run(ctx: &RunContext, initial_query: Option<String>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    std::fs::create_dir_all(&ctx.root)?;
    
    let mut app = App::new(ctx.root.clone(), initial_query.unwrap_or_default())?;
    let result = event_loop(&mut terminal, &mut app, ctx);
    
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<ratatui::backend::CrosstermBackend<Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<Stdout>>,
) -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

struct App {
    root: PathBuf,
    all_entries: Vec<PathBuf>,
    filtered_entries: Vec<PathBuf>,
    selected: usize,
    query: String,
    scroll: usize,
}

impl App {
    fn new(root: PathBuf, query: String) -> Result<Self> {
        let all_entries = list_entries(&root)?;
        let mut app = Self {
            root,
            all_entries,
            filtered_entries: Vec::new(),
            selected: 0,
            query,
            scroll: 0,
        };
        app.apply_filter();
        Ok(app)
    }

    fn apply_filter(&mut self) {
        let query = self.query.to_lowercase();
        self.filtered_entries = self.all_entries
            .iter()
            .filter(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().to_lowercase().contains(&query))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        self.clamp_selection();
    }

    fn clamp_selection(&mut self) {
        if self.filtered_entries.is_empty() {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(self.filtered_entries.len().saturating_sub(1));
        }
    }

    fn ensure_selection_visible(&mut self, list_height: usize) {
        if self.filtered_entries.is_empty() {
            return;
        }
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + list_height {
            self.scroll = self.selected - list_height + 1;
        }
    }

    fn delete_selected(&mut self) -> Result<()> {
        if let Some(path) = self.filtered_entries.get(self.selected) {
            std::fs::remove_dir_all(path)?;
            self.all_entries = list_entries(&self.root)?;
            self.apply_filter();
        }
        Ok(())
    }
}

fn list_entries(root: &Path) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(root)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .collect();
    entries.sort_by(|a, b| b.cmp(a));
    Ok(entries)
}

fn event_loop(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<Stdout>>,
    app: &mut App,
    ctx: &RunContext,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break Ok(()),
                    (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        app.selected = app.selected.saturating_sub(1);
                        app.clamp_selection();
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        if app.selected + 1 < app.filtered_entries.len() {
                            app.selected += 1;
                        }
                        app.clamp_selection();
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(path) = app.filtered_entries.get(app.selected) {
                            ctx.print_cd(path);
                        }
                        break Ok(());
                    }
                    (KeyCode::Backspace, _) => {
                        app.query.pop();
                        app.apply_filter();
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        app.delete_selected()?;
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE) if !c.is_control() => {
                        app.query.push(c);
                        app.apply_filter();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, chunks[0], &app.query);
    render_list(f, chunks[1], app);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, query: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Try Selector ")
        .border_style(Style::default().fg(Color::Cyan));
    
    let content = Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(query),
    ]);
    
    f.render_widget(Paragraph::new(content).block(block), area);
}

fn render_list(f: &mut Frame, area: Rect, app: &mut App) {
    let list_height = area.height.saturating_sub(2) as usize;
    app.ensure_selection_visible(list_height);

    let start = app.scroll;
    let end = (app.scroll + list_height).min(app.filtered_entries.len());
    
    let lines: Vec<Line> = app.filtered_entries[start..end]
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name = path.file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();
            
            let idx = start + i;
            let is_selected = idx == app.selected;
            
            let (prefix, style) = if is_selected {
                ("→ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ("  ", Style::default())
            };
            
            Line::from(vec![
                Span::styled(prefix, style),
                Span::styled("📁 ", style),
                Span::styled(name, style),
            ])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} results ", app.filtered_entries.len()));
        
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let content = "↑↓/Ctrl-PN: Navigate  Enter: Select  Ctrl-D: Delete  Esc: Cancel";
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
        
    f.render_widget(
        Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::Gray)),
        area
    );
}
