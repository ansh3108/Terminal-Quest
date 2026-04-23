use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Gauge, List, ListItem},
    Frame,
};
use crate::app::App;

const MONSTERS: [&str; 3] = [
    r#"
      / \__
     (    @\___
     /         O
    /   (_____/
    /_____/   U
    "#,
    r#"
     .-"-.
    / 4 4 \
    \_ v _/
    //   \\
    ((   ))
    -^^--^^-
    "#,
    r#"
      _---_
     /     \
    | () () |
     \  ^  /
      |||||
      |||||
    "#
];

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(6)])
        .split(f.size());

    let hp_color = if app.distraction_timer > 10 { Color::Red } else { Color::Green };
    
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" VITAL SIGNS ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(hp_color))
            .percent((app.character.hp as f64 / app.character.max_hp as f64 * 100.0) as u16),
        chunks[0],
    );

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // Inventory Sidebar
    let items: Vec<ListItem> = app.character.inventory.iter()
        .map(|i| ListItem::new(format!("+ {}", i.name)).style(Style::default().fg(Color::Yellow)))
        .collect();
    f.render_widget(List::new(items).block(Block::default().title(" ITEMS ").borders(Borders::ALL)), mid_chunks[0]);

    // Boss Stage
    if let Some(boss) = &app.current_boss {
        let b_hp = (boss.hp / boss.max_hp * 100.0).max(0.0) as u16;
        
        let mut display = format!("\n[ {} ]\n", boss.name.to_uppercase());
        display.push_str(MONSTERS[boss.monster_type]);
        
        if app.distraction_timer > 0 {
            display.push_str(&format!("\n\n!!! DANGER: {}s !!!", (10 - app.distraction_timer.min(10))));
        }

        f.render_widget(
            Paragraph::new(display)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(if app.distraction_timer > 0 { Color::Red } else { Color::White })),
            mid_chunks[1]
        );
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(4).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" LOG ").borders(Borders::ALL)), chunks[2]);
}