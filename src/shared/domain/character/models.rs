use serde::{Deserialize, Serialize};
use crate::domain::shared::models::{Position, Stats, CombatStats, Direction};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub usersname: String,
    pub gender: String, // 'male' or 'female'
    pub class: PlayerClass,
    pub level: i32,
    pub exp: i64,
    pub exp_to_next_level: i64,
    
    // 스탯
    pub stats: Stats,
    pub stat_points: i32,
    
    // 전투 스탯
    pub combat_stats: CombatStats,
    
    // 장비 및 인벤토리
    pub equipment: std::collections::HashMap<crate::shared::domain::item::models::EquipmentSlot, crate::shared::domain::item::models::Item>,
    pub inventory: Vec<Option<crate::shared::domain::item::models::Item>>, // 24 slots, some maybe empty
    pub gold: i64,
    pub position: Position,
    pub direction: Direction,
    
    // 재화
    pub gold: i64,
    
    // 상태
    pub is_moving: bool,
    pub is_attacking: bool,
    pub target_monster_id: Option<String>,
    pub last_attack_time: f64,
    pub attack_cooldown: f64,
}

impl Player {
    pub fn new(username: String, class: PlayerClass) -> Self {
        let stats = class.get_base_stats();
        let combat_stats = CombatStats::from_stats(&stats, 1);
        
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            gender: "male".to_string(),
            class,
            level: 1,
            exp: 0,
            exp_to_next_level: 100,
            stats,
            stat_points: 0,
            stat_points: 0,
            combat_stats,
            equipment: std::collections::HashMap::new(),
            inventory: vec![None; 24], // 24 slots
            current_map: "village".to_string(),
            position: Position::new(400.0, 300.0),
            direction: Direction::Down,
            gold: 100,
            is_moving: false,
            is_attacking: false,
            target_monster_id: None,
            last_attack_time: 0.0,
            attack_cooldown: 1000.0,
        }
    }

    pub fn can_attack(&self, current_time: f64) -> bool {
        current_time - self.last_attack_time >= self.attack_cooldown
    }

    pub fn register_attack(&mut self, current_time: f64) {
        self.last_attack_time = current_time;
        self.is_attacking = true;
    }
    
    pub fn add_exp(&mut self, amount: i64) {
        self.exp += amount;
        while self.exp >= self.exp_to_next_level {
            self.level_up();
        }
    }
    
    fn level_up(&mut self) {
        self.exp -= self.exp_to_next_level;
        self.level += 1;
        self.exp_to_next_level = self.calculate_exp_to_next_level();
        self.stat_points += 2; // User requested 2 points per level
        
        // 레벨업 시 HP/MP 회복
        self.combat_stats = CombatStats::from_stats(&self.stats, self.level);
    }
    
    fn calculate_exp_to_next_level(&self) -> i64 {
        (100.0 * (self.level as f64).powf(1.5)) as i64
    }
    
    pub fn add_stat(&mut self, stat_type: StatType, amount: i32) {
        if self.stat_points >= amount {
            match stat_type {
                StatType::Str => self.stats.str += amount,
                StatType::Dex => self.stats.dex += amount,
                StatType::Int => self.stats.int += amount,
                StatType::Con => self.stats.con += amount,
                StatType::Wis => self.stats.wis += amount,
            }
            self.stat_points -= amount;
            self.combat_stats = CombatStats::from_stats(&self.stats, self.level);
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        self.combat_stats.hp = (self.combat_stats.hp - damage).max(0);
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.combat_stats.hp = (self.combat_stats.hp + amount).min(self.combat_stats.max_hp);
    }
    
    pub fn is_dead(&self) -> bool {
        self.combat_stats.hp <= 0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlayerClass {
    Warrior,
    Rogue,
    Mage,
    Cleric,
    MartialArtist,
}

impl PlayerClass {
    pub fn name(&self) -> &str {
        match self {
            PlayerClass::Warrior => "전사",
            PlayerClass::Rogue => "도적",
            PlayerClass::Mage => "마법사",
            PlayerClass::Cleric => "성직자",
            PlayerClass::MartialArtist => "무도가",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            PlayerClass::Warrior => "강인한 체력과 파괴력을 지닌 전사",
            PlayerClass::Rogue => "빠른 몸놀림과 기습에 능한 도적",
            PlayerClass::Mage => "강력한 마법으로 적을 섬멸하는 마법사",
            PlayerClass::Cleric => "신성한 힘으로 아군을 치유하는 성직자",
            PlayerClass::MartialArtist => "극한의 신체 능력을 지닌 무도가",
        }
    }
    
    pub fn get_base_stats(&self) -> Stats {
        // Legend of Darkness style roughly
        match self {
            PlayerClass::Warrior => Stats {
                str: 10, dex: 5, int: 3, con: 10, wis: 3
            },
            PlayerClass::Rogue => Stats {
                str: 7, dex: 10, int: 3, con: 5, wis: 5
            },
            PlayerClass::Mage => Stats {
                str: 3, dex: 4, int: 10, con: 3, wis: 3
            },
            PlayerClass::Cleric => Stats {
                str: 4, dex: 4, int: 7, con: 5, wis: 3 // Wis -> Int/Vit mix
            },
            PlayerClass::MartialArtist => Stats {
                str: 8, dex: 8, int: 3, con: 8, wis: 3
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatType {
    Str,
    Dex,
    Int,
    Con,
    Wis,
}
