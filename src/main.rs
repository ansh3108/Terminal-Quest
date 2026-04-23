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

    #[arg(short, long, default_value_t = 30)]
    time: u32, // Minutes
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::load().unwrap_or_else(|_| App::new());
    app.start_boss(&args.task_name, args.time);

    let (tx, rx) = std::sync::mpsc::channel();
    watcher::start_watcher(tx);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        while let Ok(event) = rx.try_recv() {
            match event {
                Event::DistractionDetected => {
                    app.track_distraction();
                }
                Event::FocusPulse => {
                    app.hit_boss(0.1); // Scaled for longer tasks
                    app.reset_distraction();
                }
                Event::Tick => {
                    app.tick();
                }
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

        if matches!(app.status, GameStatus::Victorious) || matches!(app.status, GameStatus::Defeated) {
            app.save()?;
            thread::sleep(Duration::from_secs(3));
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
} 