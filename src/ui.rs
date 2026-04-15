use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Gauge, List, ListItem},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(10),
        ])
        .split(f.size());

    let hp_color = if app.character.hp > 30 { Color::Green } else { Color::Red };
    let hp_gauge = Gauge::default()
        .block(Block::default().title(" [ PLAYER HP ] ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(hp_color))
        .percent((app.character.hp as f64 / app.character.max_hp as f64 * 100.0) as u16);
    
    f.render_widget(hp_gauge, chunks[0]);

    if let Some(boss) = &app.current_boss {
        let boss_hp_percent = (boss.hp / boss.max_hp * 100.0).max(0.0) as u16;
        let boss_gauge = Gauge::default()
            .block(Block::default().title(format!(" [ BOSS: {} ] ", boss.name)).borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(boss_hp_percent);
        f.render_widget(boss_gauge, chunks[1]);
    }

    let logs: Vec<ListItem> = app.logs.iter().rev()
        .map(|log| ListItem::new(log.as_str()).style(Style::default().fg(Color::Indexed(2))))
        .collect();
    
    let log_list = List::new(logs)
        .block(Block::default().title(" [ BATTLE LOG ] ").borders(Borders::ALL));
    
    f.render_widget(log_list, chunks[2]);
}