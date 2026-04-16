mod app;
mod ui;
mod watcher;

use clap::Parser;
use std::{error::Error, io, thread, time::Duration};
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
    #[arg(default_value = "The Procrastination Demon")]
    task_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // Try to load existing character, or make a new one
    let mut app = App::load().unwrap_or_else(|_| App::new());
    app.start_boss(&args.task_name);

    let (tx, rx) = std::sync::mpsc::channel();
    watcher::start_watcher(tx);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        while let Ok(event) = rx.try_recv() {
            match event {
                Event::DistractionDetected => {
                    app.take_damage(1);
                }
                Event::FocusPulse => {
                    app.hit_boss(0.5);
                }
                Event::Tick => {}
            }
        }

        if event::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.save()?;
                    break;
                }
            }
        }

        if matches!(app.status, GameStatus::Victorious) {
            app.save()?;
            thread::sleep(Duration::from_secs(2));
            break;
        }

        if matches!(app.status, GameStatus::Defeated) {
            thread::sleep(Duration::from_secs(3));
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}