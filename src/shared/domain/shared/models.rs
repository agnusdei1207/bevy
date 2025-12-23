use serde::{Deserialize, Serialize};

/// Base stats for characters
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stats {
    pub str_stat: i32,  // Strength - 물리 공격력
    pub dex_stat: i32,  // Dexterity - 크리티컬, 회피
    pub int_stat: i32,  // Intelligence - 마법 공격력
    pub wis_stat: i32,  // Wisdom - MP, 마법 방어력
    pub con_stat: i32,  // Constitution - HP, 물리 방어력
}

impl Stats {
    pub fn new(str_stat: i32, dex_stat: i32, int_stat: i32, wis_stat: i32, con_stat: i32) -> Self {
        Self { str_stat, dex_stat, int_stat, wis_stat, con_stat }
    }
    
    /// Create stats with default values
    pub fn default_stats() -> Self {
        Self::default()
    }
    
    /// Total stat points
    pub fn total(&self) -> i32 {
        self.str_stat + self.dex_stat + self.int_stat + self.wis_stat + self.con_stat
    }
}

impl std::ops::Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            str_stat: self.str_stat + other.str_stat,
            dex_stat: self.dex_stat + other.dex_stat,
            int_stat: self.int_stat + other.int_stat,
            wis_stat: self.wis_stat + other.wis_stat,
            con_stat: self.con_stat + other.con_stat,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            str_stat: 10,
            dex_stat: 10,
            int_stat: 10,
            wis_stat: 10,
            con_stat: 10,
        }
    }
}

/// Combat-derived stats calculated from base stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub magic_attack: i32,
    pub magic_defense: i32,
    pub hit_rate: i32,
    pub avoid_rate: i32,
    pub critical_rate: i32,
}

impl CombatStats {
    pub fn from_stats(stats: &Stats, level: i32) -> Self {
        // HP: Base + Level bonus + CON bonus
        let base_hp = 100 + level * 20;
        let hp_multiplier = 1.0 + (stats.con_stat as f32 * 0.01);
        let max_hp = (base_hp as f32 * hp_multiplier) as i32;

        // MP: Base + Level bonus + WIS bonus
        let base_mp = 50 + level * 10;
        let mp_multiplier = 1.0 + (stats.wis_stat as f32 * 0.01);
        let max_mp = (base_mp as f32 * mp_multiplier) as i32;
        
        // Physical Attack: STR bonus
        let base_attack_min = 5 + level;
        let base_attack_max = 10 + level;
        let str_multiplier = 1.0 + (stats.str_stat as f32 * 0.01);
        let attack_min = (base_attack_min as f32 * str_multiplier) as i32;
        let attack_max = (base_attack_max as f32 * str_multiplier) as i32;

        // Magic Attack: INT bonus
        let base_magic_attack = 5 + level;
        let int_multiplier = 1.0 + (stats.int_stat as f32 * 0.01);
        let magic_attack = (base_magic_attack as f32 * int_multiplier) as i32;
        
        // Critical Rate: DEX bonus (0.1% per point)
        let critical_rate = 5 + (stats.dex_stat as f32 * 0.1) as i32;

        Self {
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            attack_min,
            attack_max,
            defense: 5 + stats.con_stat / 2,
            magic_attack,
            magic_defense: 5 + stats.wis_stat / 2,
            hit_rate: 80 + stats.dex_stat,
            avoid_rate: 10 + stats.dex_stat / 2,
            critical_rate,
        }
    }
}

/// Position in the game world
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Movement direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
