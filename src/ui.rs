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
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(6)])
        .split(f.size());

    // Header with dynamic color if distracted
    let header_color = if app.distraction_timer > 0 { Color::Red } else { Color::Green };
    let hp_pct = (app.character.hp as f64 / app.character.max_hp as f64 * 100.0) as u16;
    
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" STATUS ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(header_color))
            .percent(hp_pct),
        chunks[0],
    );

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // Inventory
    let items: Vec<ListItem> = app.character.inventory.iter()
        .map(|i| ListItem::new(format!("> {}", i.name)).style(Style::default().fg(Color::Yellow)))
        .collect();
    f.render_widget(List::new(items).block(Block::default().title(" GEAR ").borders(Borders::ALL)), mid_chunks[0]);

    // Boss & Danger Alert
    if let Some(boss) = &app.current_boss {
        let b_hp = (boss.hp / boss.max_hp * 100.0).max(0.0) as u16;
        let mut boss_text = format!("\n\n   TARGET: {}\n   HP: {}%", boss.name, b_hp);
        
        if app.distraction_timer > 0 {
            boss_text.push_str(&format!("\n\n   !!! TRAP ACTIVE: {}s !!!", 10 - app.distraction_timer.min(10)));
        }

        f.render_widget(
            Paragraph::new(boss_text)
                .block(Block::default().title(" ENCOUNTER ").borders(Borders::ALL))
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().fg(if app.distraction_timer > 5 { Color::Red } else { Color::White })),
            mid_chunks[1],
        );
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(4).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" LOG ").borders(Borders::ALL)), chunks[2]);
}