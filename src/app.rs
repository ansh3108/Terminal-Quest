use serde::{Serialize, Deserialize};
use std::fs;
use notify_rust::Notification;
use crate::audio;
use crate::webhook;

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
    #[default]
    Resting,
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
    #[serde(skip)]
    pub focus_streak: u32,
    #[serde(skip)]
    pub pomodoro_break: bool,
    #[serde(skip)]
    pub break_timer: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            character: Character {
                hp: 100, max_hp: 100, xp: 0, level: 1, gold: 0, inventory: vec![],
                bosses_defeated: 0, focus_pulses: 0,
            },
            config: Self::load_config(),
            current_boss: None,
            status: GameStatus::Resting,
            logs: vec!["System Initialized.".into()],
            distraction_timer: 0,
            focus_streak: 0,
            pomodoro_break: false,
            break_timer: 0,
        }
    }

    fn load_config() -> AppConfig {
        if let Ok(data) = fs::read_to_string("config.toml") {
            toml::from_str(&data).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    pub fn set_status(&mut self, new_status: GameStatus) {
        if self.status == GameStatus::Battling && new_status != GameStatus::Battling {
            return; // Can't open menus mid-battle
        }
        self.status = new_status;
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
        self.focus_streak = 0;
        self.logs.push(format!("Engaging: {}", name));
    }

    pub fn register_focus(&mut self, base_damage: f32) {
        self.character.focus_pulses += 1;
        self.distraction_timer = 0;
        
        if self.status != GameStatus::Battling { return; }

        if self.pomodoro_break {
            self.logs.push("SHIELD OF REST is active! Your attacks bounce off. Go drink water!".into());
            return;
        }

        self.focus_streak += 1;
        // 1500 pulses is roughly 25 minutes of continuous typing
        if self.focus_streak >= 1500 {
            self.pomodoro_break = true;
            self.break_timer = 300; // 5 minute break
            self.logs.push("POMODORO SHIELD ACTIVATED! 5 minute mandatory break.".into());
            let _ = Notification::new().summary("MANDATORY BREAK").body("Step away from the keyboard!").show();
            return;
        }
        
        if self.config.audio_enabled { audio::play_hit(); }

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
        
        let gold_earned = if let Some(boss) = &self.current_boss { (boss.max_hp / 10.0) as u32 * 5 } else { 50 };
        self.character.gold += gold_earned;

        if self.character.xp >= self.character.level * 100 {
            self.character.level += 1;
            self.character.hp = self.character.max_hp;
            self.logs.push("LEVEL UP! HP Fully Restored.".into());
        }

        if self.config.audio_enabled { audio::play_victory(); }
        
        if let Some(boss) = &self.current_boss {
            webhook::send_victory_message(
                self.config.discord_webhook_url.clone(), 
                boss.name.clone(), 
                self.character.level
            );
        }

        let elixir = Item { name: "Caffeine Elixir".into(), item_type: ItemType::Elixir, power: 0.0 };
        self.character.inventory.push(elixir);
        self.logs.push(format!("Target eliminated. Found {} Gold and an Elixir.", gold_earned));
        let _ = Notification::new().summary("VICTORY").body("Boss defeated!").show();
    }

    pub fn buy_item(&mut self, item_id: u8) {
        match item_id {
            1 if self.character.gold >= 100 => {
                self.character.gold -= 100;
                self.character.max_hp += 50;
                self.character.hp += 50;
                self.logs.push("Bought Heavy Armor: Max HP +50".into());
            }
            2 if self.character.gold >= 150 => {
                self.character.gold -= 150;
                self.character.inventory.push(Item { name: "Mechanical Switches".into(), item_type: ItemType::Weapon, power: 0.5 });
                self.logs.push("Bought Mechanical Switches: Damage +0.5".into());
            }
            3 if self.character.gold >= 75 => {
                self.character.gold -= 75;
                self.character.inventory.push(Item { name: "Siren's Mute".into(), item_type: ItemType::Shield, power: 0.2 });
                self.logs.push("Bought Siren's Mute: Distraction Damage -20%".into());
            }
            _ => { self.logs.push("Not enough Gold or invalid item.".into()); }
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        if self.pomodoro_break { return; } // Immune to traps during break

        let shield_reduction: f32 = self.character.inventory.iter()
            .filter(|i| i.item_type == ItemType::Shield)
            .map(|i| i.power)
            .sum();

        let reduced = (amount as f32 * (1.0 - shield_reduction)).max(1.0) as u32;
        self.character.hp = self.character.hp.saturating_sub(reduced);
        
        if self.config.audio_enabled { audio::play_damage(); }

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
        if self.pomodoro_break { return; }
        
        self.distraction_timer += 1;
        self.focus_streak = 0; // Distractions reset pomodoro streak

        if self.distraction_timer == self.config.grace_period_seconds + 1 {
            let _ = Notification::new().summary("TRAP SPRUNG").body("Return to terminal!").show();
        }
        if self.distraction_timer > self.config.grace_period_seconds {
            self.take_damage(2);
        }
    }

    pub fn tick(&mut self) {
        if self.pomodoro_break {
            self.break_timer = self.break_timer.saturating_sub(1);
            if self.break_timer == 0 {
                self.pomodoro_break = false;
                self.focus_streak = 0;
                self.logs.push("Shield lowered. You may resume your quest.".into());
                let _ = Notification::new().summary("BREAK OVER").body("Time to get back to work!").show();
            }
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