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
            Constraint::Length(3), // Top Bar
            Constraint::Min(10),   // Middle
            Constraint::Length(7), // Log
        ])
        .split(f.size());

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Stats & Inventory
            Constraint::Percentage(70), // Boss
        ])
        .split(chunks[1]);

    let hp_pct = (app.character.hp as f64 / app.character.max_hp as f64 * 100.0) as u16;
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(format!(" LVL {} ", app.character.level)).borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(hp_pct),
        chunks[0],
    );

    let items: Vec<ListItem> = app.character.inventory.iter()
        .map(|i| ListItem::new(format!("• {}", i.name)).style(Style::default().fg(Color::Yellow)))
        .collect();
    
    f.render_widget(
        List::new(items).block(Block::default().title(" ARMORY ").borders(Borders::ALL)),
        mid_chunks[0],
    );

    if let Some(boss) = &app.current_boss {
        let b_hp = (boss.hp / boss.max_hp * 100.0).max(0.0) as u16;
        let boss_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(mid_chunks[1]);

        f.render_widget(
            Gauge::default()
                .block(Block::default().title(format!(" BOSS: {} ", boss.name)).borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Red))
                .percent(b_hp),
            boss_area[0],
        );

        let boss_art = r#"
           __      __
          (  \____/  )
           \        /
            |  @  @ |
            \   V   /
             \_____/
        "#;
        f.render_widget(Paragraph::new(boss_art).alignment(ratatui::layout::Alignment::Center), boss_area[1]);
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(5)
        .map(|l| ListItem::new(l.as_str()))
        .collect();
    f.render_widget(
        List::new(logs).block(Block::default().title(" BATTLE LOG ").borders(Borders::ALL)),
        chunks[2],
    );
}