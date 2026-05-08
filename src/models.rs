use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub blacklist: Vec<String>,
    pub grace_period_seconds: u32,
    pub base_heal_amount: u32,
    pub audio_enabled: bool,
    pub discord_webhook_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            blacklist: vec!["chrome".into(), "discord".into(), "spotify".into()],
            grace_period_seconds: 10,
            base_heal_amount: 30,
            audio_enabled: true,
            discord_webhook_url: "".into(),
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
    #[default] Resting,
    Dashboard,
    Merchant,
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
    pub gold: u32,
    pub inventory: Vec<Item>,
    pub bosses_defeated: u32,
    pub focus_pulses: u32,
    pub focus_history: Vec<(String, u64)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Boss {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
    pub monster_type: usize,
}