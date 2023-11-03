use std::io::{stdout, Stdout};

use anyhow::{Context, Result};
use battery::{Battery, Manager};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()?;
    app.clear()?;
    Ok(())
}

struct App {
    manager: Manager,
    battery: Battery,
    should_quit: bool,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let manager = battery::Manager::new()?;
        let battery = manager.batteries()?.next().context("no battery found")??;

        Ok(Self {
            terminal,
            battery,
            manager,
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
                let percent = self.battery.state_of_charge().value * 100.;
                let percent = percent.to_string();

                let rect = centered_rect(frame.size(), percent.len() as u16, 1);
                frame.render_widget(Paragraph::new(percent), rect);
            })?;

            self.handle_events()?;

            // self.manager.refresh(&mut self.battery)?;
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
