use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ItemType {
    Shield,   
    Weapon,   
    Consumable, 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub power: f32,
}

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
    pub inventory: Vec<Item>,
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
        self.logs.push(format!("Quest Started: {}", name));
    }

    pub fn take_damage(&mut self, base_amount: u32) {
        let shield_power: f32 = self.character.inventory.iter()
            .filter(|i| i.item_type == ItemType::Shield)
            .map(|i| i.power)
            .sum();

        let reduction = (base_amount as f32 * shield_power).min(base_amount as f32);
        let final_damage = (base_amount as f32 - reduction).round() as u32;

        self.character.hp = self.character.hp.saturating_sub(final_damage);
        
        if final_damage > 0 {
            self.logs.push(format!("Took {} damage (Blocked {})", final_damage, reduction));
        }

        if self.character.hp == 0 {
            self.status = GameStatus::Defeated;
        }
    }

    pub fn hit_boss(&mut self, base_damage: f32) {
        let weapon_bonus: f32 = self.character.inventory.iter()
            .filter(|i| i.item_type == ItemType::Weapon)
            .map(|i| i.power)
            .sum();

        let total_damage = base_damage + weapon_bonus;

        if let Some(ref mut boss) = self.current_boss {
            boss.hp -= total_damage;
            if boss.hp <= 0.0 {
                self.status = GameStatus::Victorious;
                self.process_victory();
            }
        }
    }

    fn process_victory(&mut self) {
        self.character.xp += 100;
        
        // Random loot drop
        let loot_pool = vec![
            Item { name: "Caffeine Shield".into(), item_type: ItemType::Shield, power: 0.2 },
            Item { name: "Mechanical Sword".into(), item_type: ItemType::Weapon, power: 0.5 },
            Item { name: "Noise-Canceling Helm".into(), item_type: ItemType::Shield, power: 0.1 },
        ];

        let dropped_item = loot_pool[self.character.xp as usize % loot_pool.len()].clone();
        self.logs.push(format!("LOOT FOUND: {}", dropped_item.name));
        self.character.inventory.push(dropped_item);

        if self.character.xp >= self.character.level * 500 {
            self.character.level += 1;
            self.character.hp = self.character.max_hp;
            self.logs.push("LEVEL UP! HP Restored.".into());
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
        app.logs = vec!["State restored.".into()];
        Ok(app)
    }
}