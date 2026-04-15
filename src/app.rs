pub enum GameStatus {
    Battling,
    Victorious,
    Defeated,
}

pub struct Character {
    pub hp: u32,
    pub max_hp: u32,
    pub xp: u32,
    pub level: u32,
}

pub struct Boss {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
}

pub struct App {
    pub character: Character,
    pub current_boss: Option<Boss>,
    pub status: GameStatus,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            character: Character {
                hp: 100,
                max_hp: 100,
                xp: 0,
                level: 1,
            },
            current_boss: Some(Boss {
                name: "The Infinite Meeting Hydra".to_string(),
                hp: 100.0,
                max_hp: 100.0,
            }),
            status: GameStatus::Battling,
            logs: vec!["Quest Started: Defeat the Hydra!".to_string()],
        }
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
                self.logs.push(format!("Victory! {} has been slain.", boss.name));
            }
        }
    }
}