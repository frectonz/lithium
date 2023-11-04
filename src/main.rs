use std::io::{stdout, Stdout};

use anyhow::{Context, Result};
use battery::Battery;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge},
    Terminal,
};

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()?;
    app.clear()?;
    Ok(())
}

struct App {
    battery: Battery,
    should_quit: bool,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(Clear(ClearType::FromCursorUp))?;

        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let manager = battery::Manager::new()?;
        let battery = manager.batteries()?.next().context("no battery found")??;

        Ok(Self {
            terminal,
            battery,
            should_quit: false,
        })
    }

    fn clear(&self) -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                }
            }
        }

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.terminal.draw(|frame| {
                let percent = (self.battery.state_of_charge().value * 100.) as u16;

                let rect = centered_rect(frame.size(), 50, 30);

                let bar = Gauge::default()
                    .block(Block::default().borders(Borders::ALL))
                    .gauge_style(
                        Style::default()
                            .fg(match self.battery.state() {
                                battery::State::Charging => Color::Green,
                                _ => Color::White,
                            })
                            .bg(Color::Black)
                            .add_modifier(Modifier::ITALIC),
                    )
                    .percent(percent);

                frame.render_widget(bar, rect);
            })?;

            self.handle_events()?;

            self.battery.refresh()?;
        }

        Ok(())
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
