use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            str: 10,
            dex: 10,
            con: 10,
            int: 10,
            wis: 10,
        }
    }
}

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
        // 기본 100 + 레벨당 10 (임의) + CON * 1% (기본값 기준 보정이나 단순 수치 가산이 아닌 퍼센트? 
        // User said: "con 체력이 1% 씩증가". Usually implies BaseHP * (1 + CON/100).
        // Let's assume a Base HP calculator.
        let base_hp = 100 + level * 20;
        let hp_multiplier = 1.0 + (stats.con as f32 * 0.01);
        let max_hp = (base_hp as f32 * hp_multiplier) as i32;

        let base_mp = 50 + level * 10;
        let mp_multiplier = 1.0 + (stats.wis as f32 * 0.01);
        let max_mp = (base_mp as f32 * mp_multiplier) as i32;
        
        // STR: 물리 공격력 1% 증가
        // Attack = BaseAttack * (1 + STR%)
        let base_attack_min = 5 + level;
        let base_attack_max = 10 + level;
        let str_multiplier = 1.0 + (stats.str as f32 * 0.01);
        let attack_min = (base_attack_min as f32 * str_multiplier) as i32;
        let attack_max = (base_attack_max as f32 * str_multiplier) as i32;

        // INT: 마법 공격력 1% 증가
        let base_magic_attack = 5 + level;
        let int_multiplier = 1.0 + (stats.int as f32 * 0.01);
        let magic_attack = (base_magic_attack as f32 * int_multiplier) as i32;
        
        // DEX: 크리티컬 0.1% 증가
        // Base critical 5% + DEX * 0.1%
        let critical_rate = 5 + (stats.dex as f32 * 0.1) as i32;

        Self {
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            attack_min,
            attack_max,
            defense: 5 + stats.con / 2, // CON roughly adds def too usually
            magic_attack,
            magic_defense: 5 + stats.wis / 2, // WIS typically magic def
            hit_rate: 80 + stats.dex,
            avoid_rate: 10 + stats.dex / 2,
            critical_rate,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
