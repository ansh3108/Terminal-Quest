mod app;
mod ui;
mod watcher;

use clap::Parser;
use std::{error::Error, io, time::Duration};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use crate::app::{App, GameStatus};
use crate::watcher::Event;

#[derive(Parser)]
struct Args {
    #[arg(default_value = "The Bug Swarm")]
    task: String,
    #[arg(short, long, default_value_t = 20)]
    time: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::load().unwrap_or_else(|_| App::new());
    app.start_boss(&args.task, args.time);

    let (tx, rx) = std::sync::mpsc::channel();
    watcher::start_watcher(tx);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        while let Ok(event) = rx.try_recv() {
            match event {
                Event::DistractionDetected => { if app.status == GameStatus::Battling { app.track_distraction(); } }
                Event::FocusPulse => { app.hit_boss(0.5); app.distraction_timer = 0; }
                Event::Tick => {}
            }
        }

        if event::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => { app.save()?; break; }
                    KeyCode::Char('u') => { app.use_elixir(); }
                    KeyCode::Char('n') => { if app.status != GameStatus::Battling { app.start_boss("Next Task", 15); } }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}