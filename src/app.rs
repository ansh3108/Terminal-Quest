use serde::{Serialize, Deserialize};
use std::fs;
use notify_rust::Notification;

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
    pub monster_type: usize,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub character: Character,
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
                hp: 100,
                max_hp: 100,
                xp: 0,
                level: 1,
                inventory: vec![],
            },
            current_boss: None,
            status: GameStatus::Resting,
            logs: vec!["Welcome to the Hub.".into()],
            distraction_timer: 0,
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
        self.logs.push(format!("Quest started: {}", name));
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
            let _ = Notification::new().summary("QUEST FAILED").body("You collapsed.").show();
        }
    }

    pub fn hit_boss(&mut self, base_damage: f32) {
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
        let elixir = Item { name: "Caffeine Elixir".into(), item_type: ItemType::Elixir, power: 0.0 };
        self.character.inventory.push(elixir);
        self.logs.push("Victory! Elixir acquired.".into());
        let _ = Notification::new().summary("VICTORY").body("Boss defeated!").show();
    }

    pub fn use_elixir(&mut self) {
        if let Some(pos) = self.character.inventory.iter().position(|i| i.item_type == ItemType::Elixir) {
            self.character.inventory.remove(pos);
            self.character.hp = (self.character.hp + 30).min(self.character.max_hp);
            self.logs.push("Healed 30 HP.".into());
        }
    }

    pub fn track_distraction(&mut self) {
        self.distraction_timer += 1;
        if self.distraction_timer == 11 {
            let _ = Notification::new().summary("TRAP").body("Take cover!").show();
        }
        if self.distraction_timer > 10 {
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
        Ok(serde_json::from_str(&data)?)
    }
}