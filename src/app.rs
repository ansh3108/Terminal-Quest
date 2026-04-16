use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
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
    pub inventory: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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
    #[serde(skip_serializing, default = "default_status")]
    pub status: GameStatus,
    #[serde(skip)]
    pub logs: Vec<String>,
}

fn default_status() -> GameStatus { GameStatus::Battling }

impl App {
    pub fn new() -> Self {
        Self {
            character: Character {
                hp: 100, max_hp: 100, xp: 0, level: 1, inventory: vec![],
            },
            current_boss: None,
            status: GameStatus::Battling,
            logs: vec!["Welcome back, Hunter.".to_string()],
        }
    }

    pub fn start_boss(&mut self, name: &str) {
        self.current_boss = Some(Boss {
            name: name.to_string(),
            hp: 100.0,
            max_hp: 100.0,
        });
        self.status = GameStatus::Battling;
        self.logs.push(format!("New Quest: {}", name));
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write("save_data.json", data)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let data = fs::read_to_string("save_data.json")?;
        let mut app: App = serde_json::from_str(&data)?;
        app.logs = vec!["State loaded from disk.".to_string()];
        Ok(app)
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.character.hp = self.character.hp.saturating_sub(amount);
        if self.character.hp == 0 {
            self.status = GameStatus::Defeated;
        }
    }

    pub fn hit_boss(&mut self, damage: f32) {
        if let Some(ref mut boss) = self.current_boss {
            boss.hp -= damage;
            if boss.hp <= 0.0 {
                self.status = GameStatus::Victorious;
                self.character.xp += 50;
                self.logs.push("Target eliminated. XP gained.".to_string());
            }
        }
    }
}