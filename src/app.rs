use serde::{Serialize, Deserialize};
use std::fs;
use notify_rust::Notification;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub blacklist: Vec<String>,
    pub grace_period_seconds: u32,
    pub base_heal_amount: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            blacklist: vec!["chrome".into(), "discord".into(), "spotify".into()],
            grace_period_seconds: 10,
            base_heal_amount: 30,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ItemType { Shield, Weapon, Elixir }

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub power: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy)]
pub enum GameStatus {
    #[default]
    Resting,
    Dashboard,
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
    pub bosses_defeated: u32,
    pub focus_pulses: u32,
}

pub struct Boss {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
    pub monster_type: usize,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub character: Character,
    #[serde(skip)]
    pub config: AppConfig,
    #[serde(skip)]
    pub current_boss: Option<Boss>,
    #[serde(skip, default)]
    pub status: GameStatus,
    #[serde(skip)]
    pub logs: Vec<String>,
    #[serde(skip)]
    pub distraction_timer: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            character: Character {
                hp: 100, max_hp: 100, xp: 0, level: 1, inventory: vec![],
                bosses_defeated: 0, focus_pulses: 0,
            },
            config: Self::load_config(),
            current_boss: None,
            status: GameStatus::Resting,
            logs: vec!["System Initialized.".into()],
            distraction_timer: 0,
        }
    }

    fn load_config() -> AppConfig {
        if let Ok(data) = fs::read_to_string("config.toml") {
            toml::from_str(&data).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    pub fn toggle_dashboard(&mut self) {
        if self.status == GameStatus::Resting {
            self.status = GameStatus::Dashboard;
        } else if self.status == GameStatus::Dashboard {
            self.status = GameStatus::Resting;
        }
    }

    pub fn start_boss(&mut self, name: &str, minutes: u32) {
        let hp = minutes as f32 * 10.0;
        self.current_boss = Some(Boss {
            name: name.to_string(),
            hp,
            max_hp: hp,
            monster_type: (minutes as usize % 3),
        });
        self.status = GameStatus::Battling;
        self.logs.push(format!("Engaging: {}", name));
    }

    pub fn register_focus(&mut self, base_damage: f32) {
        self.character.focus_pulses += 1;
        self.distraction_timer = 0;
        
        if self.status != GameStatus::Battling { return; }
        
        let weapon_bonus: f32 = self.character.inventory.iter()
            .filter(|i| i.item_type == ItemType::Weapon)
            .map(|i| i.power)
            .sum();

        if let Some(ref mut boss) = self.current_boss {
            boss.hp -= base_damage + weapon_bonus;
            if boss.hp <= 0.0 {
                self.status = GameStatus::Victorious;
                self.process_victory();
            }
        }
    }

    fn process_victory(&mut self) {
        self.character.xp += 100;
        self.character.bosses_defeated += 1;
        let elixir = Item { name: "Caffeine Elixir".into(), item_type: ItemType::Elixir, power: 0.0 };
        self.character.inventory.push(elixir);
        self.logs.push("Target eliminated. Loot acquired.".into());
        let _ = Notification::new().summary("VICTORY").body("Boss defeated!").show();
    }

    pub fn take_damage(&mut self, amount: u32) {
        let shield_reduction: f32 = self.character.inventory.iter()
            .filter(|i| i.item_type == ItemType::Shield)
            .map(|i| i.power)
            .sum();

        let reduced = (amount as f32 * (1.0 - shield_reduction)).max(1.0) as u32;
        self.character.hp = self.character.hp.saturating_sub(reduced);
        
        if self.character.hp == 0 {
            self.status = GameStatus::Defeated;
            let _ = Notification::new().summary("QUEST FAILED").body("You were overwhelmed.").show();
        }
    }

    pub fn use_elixir(&mut self) {
        if let Some(pos) = self.character.inventory.iter().position(|i| i.item_type == ItemType::Elixir) {
            self.character.inventory.remove(pos);
            self.character.hp = (self.character.hp + self.config.base_heal_amount).min(self.character.max_hp);
            self.logs.push(format!("Healed {} HP.", self.config.base_heal_amount));
        }
    }

    pub fn track_distraction(&mut self) {
        self.distraction_timer += 1;
        if self.distraction_timer == self.config.grace_period_seconds + 1 {
            let _ = Notification::new().summary("TRAP SPRUNG").body("Return to terminal!").show();
        }
        if self.distraction_timer > self.config.grace_period_seconds {
            self.take_damage(2);
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write("save_data.json", data)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let data = fs::read_to_string("save_data.json")?;
        let mut app: App = serde_json::from_str(&data)?;
        app.config = Self::load_config();
        Ok(app)
    }
}