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

    let status_color = if app.pomodoro_break { Color::Cyan } 
                       else if app.distraction_timer > app.config.grace_period_seconds { Color::Red } 
                       else { Color::Green };
                       
    let header_title = format!(" LVL {} | {} GOLD ", app.character.level, app.character.gold);
    
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(header_title).borders(Borders::ALL))
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
                let mut display = format!("\n{}\n\nTARGET: {}\nHP: {}%", MONSTERS[boss.monster_type], boss.name, b_hp);
                
                if app.pomodoro_break {
                    display = format!("\n[ SHIELD OF REST ACTIVE ]\n\nTake a break!\nShield lowers in: {}s", app.break_timer);
                }

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
        GameStatus::Merchant => {
            let shop = format!(
                "\n\n[ THE MERCHANT ]\n\nYou have {} Gold.\n\n[1] Heavy Armor (100g): Max HP +50\n[2] Mechanical Switches (150g): +0.5 Damage\n[3] Siren's Mute (75g): -20% Trap Damage\n\nPress 1, 2, or 3 to buy.",
                app.character.gold
            );
            f.render_widget(Paragraph::new(shop).alignment(Alignment::Center).block(Block::default().borders(Borders::ALL)), mid[1]);
        }
        _ => {
            let mut camp = String::from("\n( ^_^) \n\nSAFE AT CAMP\n\n");
            
            if !app.quest_board.is_empty() {
                camp.push_str(&format!("[ QUEST BOARD ]\nPending Tasks: {}\nUp Next: {}\n\n", 
                    app.quest_board.len(), 
                    app.quest_board[0].name
                ));
                camp.push_str("[n] Start Next Quest\n");
            } else {
                camp.push_str("No pending quests.\n\n[n] Random Quest\n");
            }
            
            camp.push_str("[s] Stats\n[m] Merchant\n[u] Use Elixir");
            
            f.render_widget(
                Paragraph::new(camp)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL)), 
                mid[1]
            );
        }
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(4).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" LOG ").borders(Borders::ALL)), chunks[2]);
}