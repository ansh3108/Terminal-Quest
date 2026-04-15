mod app;
mod ui;
mod watcher;

use std::{error::Error, io, thread, time::Duration};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use crate::app::{App, GameStatus};
use crate::watcher::Event;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new();
    let (tx, rx) = std::sync::mpsc::channel();
    watcher::start_watcher(tx);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        while let Ok(event) = rx.try_recv() {
            match event {
                Event::DistractionDetected => {
                    app.take_damage(2);
                    app.logs.push("Warning: Distraction detected! -2 HP".to_string());
                }
                Event::FocusPulse => {
                    app.hit_boss(0.5);
                }
                Event::Tick => {}
            }
        }

        if event::poll(Duration::from_millis(16))? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if matches!(app.status, GameStatus::Defeated) {
            terminal.draw(|f| ui::render(f, &app))?;
            thread::sleep(Duration::from_secs(3));
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
} 