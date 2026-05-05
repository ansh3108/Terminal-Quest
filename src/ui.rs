use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Gauge, List, ListItem, BarChart},
    Frame,
};
use crate::app::{App, GameStatus};

const MONSTERS: [&str; 3] = [
    "  (o o)\n  / v \\\n /     \\",
    "  <O_O>\n   /|\\\n   / \\",
    "  [X_X]\n  --|--\n   / \\"
];

pub fn render(f: &mut Frame, app: &App) {
    // SCREEN SHAKE ANIMATION
    let mut main_area = f.size();
    if app.visual_shake > 0 && app.visual_shake % 2 == 0 {
        main_area.x = main_area.x.saturating_add(3); // Jerk right
        main_area.width = main_area.width.saturating_sub(3);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(6)])
        .split(main_area);

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

                // ATTACK FLASH ANIMATION
                let mut style = Style::default().fg(if app.distraction_timer > 0 { Color::Red } else { Color::White });
                if app.visual_flash > 0 {
                    style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
                }

                f.render_widget(Paragraph::new(display).alignment(Alignment::Center).style(style).block(Block::default().borders(Borders::ALL)), mid[1]);
            }
        }
        GameStatus::Dashboard => {
            let dash_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(8), Constraint::Min(8)])
                .split(mid[1]);

            let mins = app.character.focus_pulses / 60;
            let stats = format!(
                "\nBosses Slain: {}\nEstimated Focus: {} Minutes\nTotal XP: {}\n\nActive Traps: {}",
                app.character.bosses_defeated, mins, app.character.xp, app.config.blacklist.join(", ")
            );
            f.render_widget(Paragraph::new(stats).alignment(Alignment::Center).block(Block::default().title(" HALL OF RECORDS ").borders(Borders::ALL)), dash_chunks[0]);

            // THE HEATMAP (Bar Chart)
            let chart_data: Vec<(&str, u64)> = app.character.focus_history.iter().map(|(day, val)| (day.as_str(), *val)).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(" PAST 7 DAYS (Focus Mins) ").borders(Borders::ALL))
                .data(&chart_data)
                .bar_width(7)
                .bar_gap(2)
                .bar_style(Style::default().fg(Color::Green))
                .value_style(Style::default().fg(Color::Black).bg(Color::Green));
            
            f.render_widget(barchart, dash_chunks[1]);
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
                camp.push_str(&format!("[ QUEST BOARD ]\nPending Tasks: {}\nUp Next: {}\n\n", app.quest_board.len(), app.quest_board[0].name));
                camp.push_str("[n] Start Next Quest\n");
            } else {
                camp.push_str("No pending quests.\n\n[n] Random Quest\n");
            }
            camp.push_str("[s] Stats\n[m] Merchant\n[u] Use Elixir");
            f.render_widget(Paragraph::new(camp).alignment(Alignment::Center).block(Block::default().borders(Borders::ALL)), mid[1]);
        }
    }

    let logs: Vec<ListItem> = app.logs.iter().rev().take(4).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" LOG ").borders(Borders::ALL)), chunks[2]);
}