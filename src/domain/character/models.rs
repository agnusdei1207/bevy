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
    
    // 위치
    pub current_map: String,
    pub position: Position,
    pub direction: Direction,
    
    // 재화
    pub gold: i64,
    
    // 상태
    pub is_moving: bool,
    pub is_attacking: bool,
    pub target_monster_id: Option<String>,
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
            combat_stats,
            current_map: "village".to_string(), // Updated to match DB seed
            position: Position::new(400.0, 300.0),
            direction: Direction::Down,
            gold: 100,
            is_moving: false,
            is_attacking: false,
            target_monster_id: None,
        }
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
        self.stat_points += 5;
        
        // 레벨업 시 HP/MP 회복
        self.combat_stats = CombatStats::from_stats(&self.stats, self.level);
    }
    
    fn calculate_exp_to_next_level(&self) -> i64 {
        (100.0 * (self.level as f64).powf(1.5)) as i64
    }
    
    pub fn add_stat(&mut self, stat_type: StatType, amount: i32) {
        if self.stat_points >= amount {
            match stat_type {
                StatType::Strength => self.stats.strength += amount,
                StatType::Dexterity => self.stats.dexterity += amount,
                StatType::Intelligence => self.stats.intelligence += amount,
                StatType::Vitality => self.stats.vitality += amount,
                StatType::Luck => self.stats.luck += amount,
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
                strength: 10, dexterity: 5, intelligence: 3, vitality: 10, luck: 3
            },
            PlayerClass::Rogue => Stats {
                strength: 7, dexterity: 10, intelligence: 3, vitality: 5, luck: 5
            },
            PlayerClass::Mage => Stats {
                strength: 3, dexterity: 4, intelligence: 10, vitality: 3, luck: 3
            },
            PlayerClass::Cleric => Stats {
                strength: 4, dexterity: 4, intelligence: 7, vitality: 5, luck: 3 // Wis -> Int/Vit mix
            },
            PlayerClass::MartialArtist => Stats {
                strength: 8, dexterity: 8, intelligence: 3, vitality: 8, luck: 3
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Dexterity,
    Intelligence,
    Vitality,
    Luck,
}
