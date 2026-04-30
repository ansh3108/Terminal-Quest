use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Gauge, List, ListItem},
    Frame,
};
use crate::app::{App, GameStatus};

const MONSTERS: [&str; 3] = [
    "  (o o)\n  / v \\\n /     \\",
    "  <O_O>\n   /|\\\n   / \\",
    "  [X_X]\n  --|--\n   / \\"
];

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(6)])
        .split(f.size());

    let status_color = if app.distraction_timer > app.config.grace_period_seconds { Color::Red } else { Color::Green };
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(format!(" LVL {} ", app.character.level)).borders(Borders::ALL))
            .gauge_style(Style::default().fg(status_color))
            .percent((app.character.hp as f64 / app.character.max_hp as f64 * 100.0) as u16),
        chunks[0],
    );

    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let items: Vec<ListItem> = app.character.inventory.iter()
        .map(|i| ListItem::new(format!("+ {}", i.name)).style(Style::default().fg(Color::Yellow)))
        .collect();
    f.render_widget(List::new(items).block(Block::default().title(" ARMORY ").borders(Borders::ALL)), mid[0]);

    match app.status {
        GameStatus::Battling => {
            if let Some(boss) = &app.current_boss {
                let b_hp = (boss.hp / boss.max_hp * 100.0).max(0.0) as u16;
                let display = format!("\n{}\n\nTARGET: {}\nHP: {}%", MONSTERS[boss.monster_type], boss.name, b_hp);
                f.render_widget(Paragraph::new(display).alignment(Alignment::Center).block(Block::default().borders(Borders::ALL)), mid[1]);
            }
        }
        GameStatus::Dashboard => {
            let mins = app.character.focus_pulses / 60;
            let stats = format!(
                "\n\n[ HALL OF RECORDS ]\n\nBosses Slain: {}\nEstimated Focus: {} Minutes\nTotal XP: {}\n\nActive Traps: {}",
                app.character.bosses_defeated, mins, app.character.xp, app.config.blacklist.join(", ")
            );
            f.render_widget(Paragraph::new(stats).alignment(Alignment::Center).block(Block::default().borders(Borders::ALL)), mid[1]);
        }
        _ => {
            f.render_widget(Paragraph::new("\n( ^_^) \n\nSAFE AT CAMP\nPress 'n' for new quest.\nPress 's' for stats.").alignment(Alignment::Center).block(Block::default().borders(Borders::ALL)), mid[1]);
        }
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(4).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" LOG ").borders(Borders::ALL)), chunks[2]);
}