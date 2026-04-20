use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ItemType { Shield, Weapon }

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub power: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Default)] // Added Default here
pub enum GameStatus {
    #[default] // This tells Rust to start here
    Battling,
    Victorious,
    Defeated,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub hp: u32,
    pub max_hp: u32,
    pub xp: u32,
    pub level: u32,
    pub inventory: Vec<Item>,
}

pub struct Boss {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub character: Character,
    #[serde(skip)]
    pub current_boss: Option<Boss>,
    #[serde(skip_serializing, default)] // Simplified this to just use the trait
    pub status: GameStatus,
    #[serde(skip)]
    pub logs: Vec<String>,
    #[serde(skip)]
    pub distraction_timer: u32,
    #[serde(skip)]
    pub is_distracted: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            character: Character { 
                hp: 100, 
                max_hp: 100, 
                xp: 0, 
                level: 1, 
                inventory: vec![] 
            },
            current_boss: None,
            status: GameStatus::Battling,
            logs: vec!["System initialized.".into()],
            distraction_timer: 0,
            is_distracted: false,
        }
    }

    pub fn start_boss(&mut self, name: &str, minutes: u32) {
        let estimated_hp = minutes as f32 * 6.0; 
        self.current_boss = Some(Boss {
            name: name.to_string(),
            hp: estimated_hp,
            max_hp: estimated_hp,
        });
        self.status = GameStatus::Battling;
        self.logs.push(format!("Quest: {}. Estimated time: {}m", name, minutes));
    }

    pub fn track_distraction(&mut self) {
        self.is_distracted = true;
        self.distraction_timer += 1;
        
        if self.distraction_timer > 10 {
            self.take_damage(2);
        }
    }

    pub fn reset_distraction(&mut self) {
        self.is_distracted = false;
        self.distraction_timer = 0;
    }

    pub fn tick(&mut self) {}

    pub fn take_damage(&mut self, amount: u32) {
        self.character.hp = self.character.hp.saturating_sub(amount);
        if self.character.hp == 0 { self.status = GameStatus::Defeated; }
    }

    pub fn hit_boss(&mut self, damage: f32) {
        if let Some(ref mut boss) = self.current_boss {
            boss.hp -= damage;
            if boss.hp <= 0.0 {
                self.status = GameStatus::Victorious;
                self.process_victory();
            }
        }
    }

    fn process_victory(&mut self) {
        self.character.xp += 100;
        self.logs.push("Quest Complete! You gained 100 XP.".into());
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write("save_data.json", data)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let data = fs::read_to_string("save_data.json")?;
        let mut app: App = serde_json::from_str(&data)?;
        app.logs = vec!["Progress restored from save file.".into()];
        Ok(app)
    }
}