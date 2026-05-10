use std::fs;
use crate::models::Boss;

pub fn parse_markdown_tasks(filepath: &str) -> Result<(Vec<Boss>, usize), String> {
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return Err(format!("Could not read {}. Does it exist?", filepath)),
    };

    let mut board = Vec::new();
    let mut count = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [ ]") || trimmed.starts_with("* [ ]") {
            let task_name = trimmed[5..].trim().to_string();
            let mut time = 20;
            let mut name = task_name.clone();
            
            if task_name.ends_with("m)") {
                if let Some(start) = task_name.rfind('(') {
                    let time_str = &task_name[start+1..task_name.len()-2];
                    if let Ok(parsed_time) = time_str.parse::<u32>() {
                        time = parsed_time;
                        name = task_name[..start].trim().to_string();
                    }
                }
            }

            let hp = time as f32 * 10.0;
            board.push(Boss { name, hp, max_hp: hp, monster_type: (time as usize % 3) });
            count += 1;
        }
    }
    
    Ok((board, count))
}