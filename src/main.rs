use std::{io, time::Duration};

use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};

#[derive(Clone)]
struct Target {
    id: String,
    status: Status,
    hits: usize,
    emails: Vec<String>,
    platforms: Vec<String>,
}

#[derive(Clone, PartialEq)]
enum Status {
    Scanning,
    Found,
    Empty,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Scanning => write!(f, "🦅 SCANNING"),
            Status::Found => write!(f, "✅ HIT"),
            Status::Empty => write!(f, "❌ EMPTY"),
        }
    }
}

struct App {
    targets: Vec<Target>,
    current_target: usize,
    input: String,
    logs: Vec<String>,
    scanning: bool,
}

impl App {
    fn new() -> Self {
        Self {
            targets: vec![],
            current_target: 0,
            input: String::new(),
            logs: vec![
                "[SYSTEM] BLOODY-FALCON v1.0 BOOT".to_string(),
                "[SYSTEM] RECON MODULES LOADED".to_string(),
                "[SYSTEM] TERMINAL MODE: ACTIVE".to_string(),
                "[SYSTEM] ENTER TARGET IDENTIFIER".to_string(),
            ],
            scanning: false,
        }
    }

    fn add_target(&mut self, id: String) {
        self.targets.push(Target {
            id: id.clone(),
            status: Status::Empty,
            hits: 0,
            emails: vec![],
            platforms: vec![],
        });
        self.log(&format!("[+] Target added: {}", id));
    }

    fn scan_target(&mut self, index: usize) {
        if index < self.targets.len() {
            let target_id = {
                let target = &mut self.targets[index];
                target.status = Status::Scanning;
                target.id.clone()
            };
            self.scanning = true;
            self.log(&format!("🦅 SCANNING {} across 348 platforms...", target_id));
        }
    }

    fn simulate_scan(&mut self) {
        let mut log_entry = None;
        if let Some(target) = self.targets.get_mut(self.current_target) {
            let mut rng = rand::thread_rng();
            target.status = Status::Found;
            target.hits = rng.gen_range(5..30);
            target.emails = vec![
                format!("{}@example.com", target.id),
                format!("{}@outlook.com", target.id),
            ];
            target.platforms = vec![
                "GitHub".to_string(),
                "Reddit".to_string(),
                "Steam".to_string(),
                "Twitter".to_string(),
                "PSN".to_string(),
            ];
            log_entry = Some((target.id.clone(), target.hits));
        }
        if let Some((id, hits)) = log_entry {
            self.log(&format!("✅ {} - {} hits found!", id, hits));
        }
        self.scanning = false;
    }

    fn log(&mut self, msg: &str) {
        self.logs
            .push(format!("[{}] {}", Local::now().format("%H:%M:%S"), msg));
        if self.logs.len() > 10 {
            self.logs.remove(0);
        }
    }

    fn next_target(&mut self) {
        self.current_target = (self.current_target + 1) % self.targets.len().max(1);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut scan_timer = tokio::time::interval(Duration::from_secs(2));

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Enter => {
                        if app.input.trim().is_empty() {
                            if app.targets.is_empty() {
                                app.add_target("shadow".to_string());
                            } else {
                                app.scan_target(app.current_target);
                            }
                        } else {
                            app.add_target(app.input.clone());
                            app.input.clear();
                        }
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Tab => app.next_target(),
                    _ => {}
                }
            }
        }

        // Async scan simulation
        if app.scanning {
            scan_timer.tick().await;
            app.simulate_scan();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(25),
                Constraint::Percentage(40),
                Constraint::Percentage(25),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" 🦅 ", Style::default().fg(Color::Red)),
        Span::styled(
            "BLOODY-FALCON",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" v1.0 ", Style::default().fg(Color::Yellow)),
        Span::styled("348 PLATFORMS", Style::default().fg(Color::Cyan)),
        Span::raw(" | ENTER=SCAN"),
    ]))
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Red)));
    f.render_widget(title, chunks[0]);

    // Targets list
    let target_items: Vec<ListItem> = app
        .targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let status_color = match t.status {
                Status::Scanning => Color::Yellow,
                Status::Found => Color::Green,
                Status::Empty => Color::White,
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:2}", i), Style::default().fg(Color::Gray)),
                Span::raw(" | "),
                Span::styled(&t.id, Style::default().fg(status_color)),
                Span::raw(" ["),
                Span::styled(format!("Hits: {}", t.hits), Style::default().fg(Color::Cyan)),
                Span::raw("]"),
            ]))
        })
        .collect();

    let targets = List::new(target_items)
        .block(
            Block::default()
                .title(" 🦅 ACTIVE TARGETS (TAB to switch) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_widget(targets, chunks[1]);

    // Intel feed
    let placeholder = Target {
        id: "No Target".to_string(),
        status: Status::Empty,
        hits: 0,
        emails: vec![],
        platforms: vec![],
    };
    let current = app.targets.get(app.current_target).unwrap_or(&placeholder);

    let mut intel_lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("Target: ", Style::default().fg(Color::White)),
            Span::styled(
                &current.id,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(current.status.to_string(), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Hits: ", Style::default().fg(Color::White)),
            Span::styled(current.hits.to_string(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![Span::styled("Emails:", Style::default().fg(Color::White))]),
    ];

    if current.emails.is_empty() {
        intel_lines.push(Line::from(vec![Span::styled(
            "None recorded",
            Style::default().fg(Color::DarkGray),
        )]));
    } else {
        for email in &current.emails {
            intel_lines.push(Line::from(vec![Span::styled(
                email,
                Style::default().fg(Color::Magenta),
            )]));
        }
    }

    let platform_line = if current.platforms.is_empty() {
        "None".to_string()
    } else {
        current.platforms.join(", ")
    };

    intel_lines.push(Line::from(vec![
        Span::styled("Platforms: ", Style::default().fg(Color::White)),
        Span::styled(platform_line, Style::default().fg(Color::Yellow)),
    ]));

    let intel = Paragraph::new(intel_lines)
        .block(Block::default().title(" 🛡️ INTEL FEED ").borders(Borders::ALL));
    f.render_widget(intel, chunks[2]);

    // Scan progress
    if app.scanning {
        let progress = Gauge::default()
            .block(Block::default().title(" 🔍 SCAN PROGRESS ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(0.7);
        f.render_widget(progress, chunks[3]);
    } else {
        let progress = Paragraph::new("Press ENTER to start scan")
            .block(Block::default().title(" 🔍 SCAN ENGINE ").borders(Borders::ALL));
        f.render_widget(progress, chunks[3]);
    }

    // Input + logs
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[4]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().title(" 🎯 ENTER TARGET ID ").borders(Borders::ALL));
    f.render_widget(input, bottom_chunks[0]);

    let log_items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(6)
        .map(|log| ListItem::new(Line::from(vec![
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::raw(log),
        ])))
        .collect();

    let logs = List::new(log_items)
        .block(Block::default().title(" 📜 SYSTEM LOGS ").borders(Borders::ALL));
    f.render_widget(logs, bottom_chunks[1]);
}
