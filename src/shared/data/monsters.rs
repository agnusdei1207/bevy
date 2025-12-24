//! Monster data constants
//!
//! All monster definitions that were previously stored in the database.
//! Now defined as compile-time constants for fast access.

use crate::shared::domain::monster::{MonsterAIType, SpriteSize};

/// Monster definition constant data
#[derive(Debug, Clone)]
pub struct MonsterDef {
    pub id: i32,
    pub name: &'static str,
    pub name_key: &'static str, // i18n key
    pub level: i32,
    pub hp_max: i32,
    pub mp_max: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    pub ai_type: MonsterAIType,
    pub detection_range: f64,
    pub attack_range: f64,
    pub move_speed: f64,
    pub sprite_type: &'static str,
    pub sprite_size: SpriteSize,
    pub description_key: &'static str, // i18n key
}

impl MonsterDef {
    /// Get sprite sheet path for this monster
    pub fn sprite_path(&self) -> String {
        format!("/assets/monsters/{}/spritesheet.webp", self.sprite_type)
    }
}

// ============ Level 1-10 Monsters ============

pub const GIANT_RAT: MonsterDef = MonsterDef {
    id: 1,
    name: "Giant Rat",
    name_key: "monster.giant_rat",
    level: 1,
    hp_max: 20,
    mp_max: 0,
    attack_min: 2,
    attack_max: 4,
    defense: 0,
    exp_reward: 5,
    gold_min: 1,
    gold_max: 3,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 250.0,
    attack_range: 55.0,
    move_speed: 110.0,
    sprite_type: "rat",
    sprite_size: SpriteSize::Small,
    description_key: "monster.giant_rat.desc",
};

pub const VAMPIRE_BAT: MonsterDef = MonsterDef {
    id: 2,
    name: "Vampire Bat",
    name_key: "monster.vampire_bat",
    level: 3,
    hp_max: 35,
    mp_max: 0,
    attack_min: 4,
    attack_max: 6,
    defense: 1,
    exp_reward: 10,
    gold_min: 2,
    gold_max: 5,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 250.0,
    attack_range: 55.0,
    move_speed: 120.0,
    sprite_type: "bat",
    sprite_size: SpriteSize::Small,
    description_key: "monster.vampire_bat.desc",
};

pub const SLIME: MonsterDef = MonsterDef {
    id: 3,
    name: "Slime",
    name_key: "monster.slime",
    level: 5,
    hp_max: 50,
    mp_max: 0,
    attack_min: 6,
    attack_max: 9,
    defense: 2,
    exp_reward: 18,
    gold_min: 5,
    gold_max: 10,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 150.0,
    attack_range: 40.0,
    move_speed: 60.0,
    sprite_type: "slime",
    sprite_size: SpriteSize::Small,
    description_key: "monster.slime.desc",
};

// ============ Level 11-30 Monsters ============

pub const CORRUPTED_FOX: MonsterDef = MonsterDef {
    id: 4,
    name: "Corrupted Fox",
    name_key: "monster.corrupted_fox",
    level: 10,
    hp_max: 120,
    mp_max: 20,
    attack_min: 15,
    attack_max: 20,
    defense: 5,
    exp_reward: 45,
    gold_min: 15,
    gold_max: 30,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 200.0,
    attack_range: 50.0,
    move_speed: 100.0,
    sprite_type: "fox",
    sprite_size: SpriteSize::Medium,
    description_key: "monster.corrupted_fox.desc",
};

pub const WOLF: MonsterDef = MonsterDef {
    id: 5,
    name: "Wolf",
    name_key: "monster.wolf",
    level: 15,
    hp_max: 200,
    mp_max: 30,
    attack_min: 25,
    attack_max: 35,
    defense: 10,
    exp_reward: 80,
    gold_min: 25,
    gold_max: 50,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 280.0,
    attack_range: 60.0,
    move_speed: 130.0,
    sprite_type: "wolf",
    sprite_size: SpriteSize::Medium,
    description_key: "monster.wolf.desc",
};

pub const SKELETON: MonsterDef = MonsterDef {
    id: 6,
    name: "Skeleton",
    name_key: "monster.skeleton",
    level: 20,
    hp_max: 350,
    mp_max: 0,
    attack_min: 40,
    attack_max: 55,
    defense: 15,
    exp_reward: 150,
    gold_min: 40,
    gold_max: 80,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 220.0,
    attack_range: 55.0,
    move_speed: 90.0,
    sprite_type: "skeleton",
    sprite_size: SpriteSize::Medium,
    description_key: "monster.skeleton.desc",
};

// ============ Level 31-60 Monsters ============

pub const GOBLIN: MonsterDef = MonsterDef {
    id: 7,
    name: "Goblin",
    name_key: "monster.goblin",
    level: 30,
    hp_max: 600,
    mp_max: 50,
    attack_min: 60,
    attack_max: 80,
    defense: 25,
    exp_reward: 300,
    gold_min: 80,
    gold_max: 150,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 250.0,
    attack_range: 50.0,
    move_speed: 100.0,
    sprite_type: "goblin",
    sprite_size: SpriteSize::Medium,
    description_key: "monster.goblin.desc",
};

pub const GHOST: MonsterDef = MonsterDef {
    id: 8,
    name: "Ghost",
    name_key: "monster.ghost",
    level: 40,
    hp_max: 1000,
    mp_max: 200,
    attack_min: 90,
    attack_max: 120,
    defense: 10,
    exp_reward: 600,
    gold_min: 120,
    gold_max: 250,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 300.0,
    attack_range: 70.0,
    move_speed: 80.0,
    sprite_type: "ghost",
    sprite_size: SpriteSize::Large,
    description_key: "monster.ghost.desc",
};

pub const DARK_KNIGHT: MonsterDef = MonsterDef {
    id: 9,
    name: "Dark Knight",
    name_key: "monster.dark_knight",
    level: 60,
    hp_max: 3000,
    mp_max: 500,
    attack_min: 200,
    attack_max: 300,
    defense: 100,
    exp_reward: 2000,
    gold_min: 500,
    gold_max: 1000,
    ai_type: MonsterAIType::Aggressive,
    detection_range: 350.0,
    attack_range: 80.0,
    move_speed: 90.0,
    sprite_type: "dark_knight",
    sprite_size: SpriteSize::Large,
    description_key: "monster.dark_knight.desc",
};

// ============ Level 61-99 Monsters ============

pub const LICH: MonsterDef = MonsterDef {
    id: 10,
    name: "Lich",
    name_key: "monster.lich",
    level: 80,
    hp_max: 8000,
    mp_max: 5000,
    attack_min: 500,
    attack_max: 800,
    defense: 200,
    exp_reward: 10000,
    gold_min: 2000,
    gold_max: 5000,
    ai_type: MonsterAIType::Defensive,
    detection_range: 400.0,
    attack_range: 100.0,
    move_speed: 70.0,
    sprite_type: "lich",
    sprite_size: SpriteSize::Large,
    description_key: "monster.lich.desc",
};

// ============ Boss Monsters ============

pub const RED_DRAGON: MonsterDef = MonsterDef {
    id: 11,
    name: "Red Dragon",
    name_key: "monster.red_dragon",
    level: 99,
    hp_max: 50000,
    mp_max: 10000,
    attack_min: 1000,
    attack_max: 2000,
    defense: 500,
    exp_reward: 100000,
    gold_min: 10000,
    gold_max: 50000,
    ai_type: MonsterAIType::Defensive,
    detection_range: 500.0,
    attack_range: 150.0,
    move_speed: 100.0,
    sprite_type: "dragon",
    sprite_size: SpriteSize::Boss,
    description_key: "monster.red_dragon.desc",
};

/// All monster definitions
pub const ALL_MONSTERS: &[&MonsterDef] = &[
    &GIANT_RAT,
    &VAMPIRE_BAT,
    &SLIME,
    &CORRUPTED_FOX,
    &WOLF,
    &SKELETON,
    &GOBLIN,
    &GHOST,
    &DARK_KNIGHT,
    &LICH,
    &RED_DRAGON,
];

/// Get monster definition by name
pub fn get_monster_by_name(name: &str) -> Option<&'static MonsterDef> {
    ALL_MONSTERS.iter().find(|m| m.name == name).copied()
}

/// Get monster definition by ID
pub fn get_monster_by_id(id: i32) -> Option<&'static MonsterDef> {
    ALL_MONSTERS.iter().find(|m| m.id == id).copied()
}

/// Monster loot drop definition
#[derive(Debug, Clone)]
pub struct LootDropDef {
    pub monster_id: i32,
    pub item_id: i32,
    pub probability: f64,
    pub min_quantity: i32,
    pub max_quantity: i32,
}

/// Monster drop table
pub const MONSTER_DROPS: &[LootDropDef] = &[
    // Giant Rat drops Red Potion (10%)
    LootDropDef { monster_id: 1, item_id: 1, probability: 0.1, min_quantity: 1, max_quantity: 1 },
    // Vampire Bat drops Red Potion (15%)
    LootDropDef { monster_id: 2, item_id: 1, probability: 0.15, min_quantity: 1, max_quantity: 1 },
    // Slime drops Red Potion (20%) and Blue Potion (5%)
    LootDropDef { monster_id: 3, item_id: 1, probability: 0.2, min_quantity: 1, max_quantity: 1 },
    LootDropDef { monster_id: 3, item_id: 2, probability: 0.05, min_quantity: 1, max_quantity: 1 },
];

/// Get drops for a specific monster
pub fn get_monster_drops(monster_id: i32) -> Vec<&'static LootDropDef> {
    MONSTER_DROPS.iter().filter(|d| d.monster_id == monster_id).collect()
}

/// Sprite animation configuration for monsters
#[derive(Debug, Clone)]
pub struct MonsterSpriteConfig {
    pub sprite_type: &'static str,
    pub frame_width: u32,
    pub frame_height: u32,
    pub idle_frames: u32,
    pub walk_frames: u32,
    pub attack_frames: u32,
    pub death_frames: u32,
    pub animation_speed: f32,
}

pub const MONSTER_SPRITES: &[MonsterSpriteConfig] = &[
    MonsterSpriteConfig {
        sprite_type: "rat",
        frame_width: 32,
        frame_height: 32,
        idle_frames: 4,
        walk_frames: 4,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.15,
    },
    MonsterSpriteConfig {
        sprite_type: "bat",
        frame_width: 32,
        frame_height: 32,
        idle_frames: 4,
        walk_frames: 4,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.1,
    },
    MonsterSpriteConfig {
        sprite_type: "slime",
        frame_width: 32,
        frame_height: 32,
        idle_frames: 4,
        walk_frames: 4,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.2,
    },
    MonsterSpriteConfig {
        sprite_type: "wolf",
        frame_width: 48,
        frame_height: 48,
        idle_frames: 4,
        walk_frames: 6,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.12,
    },
    MonsterSpriteConfig {
        sprite_type: "skeleton",
        frame_width: 48,
        frame_height: 48,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 6,
        death_frames: 4,
        animation_speed: 0.15,
    },
    MonsterSpriteConfig {
        sprite_type: "goblin",
        frame_width: 48,
        frame_height: 48,
        idle_frames: 4,
        walk_frames: 6,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.12,
    },
    MonsterSpriteConfig {
        sprite_type: "ghost",
        frame_width: 64,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 4,
        attack_frames: 4,
        death_frames: 4,
        animation_speed: 0.2,
    },
    MonsterSpriteConfig {
        sprite_type: "dragon",
        frame_width: 128,
        frame_height: 128,
        idle_frames: 4,
        walk_frames: 6,
        attack_frames: 6,
        death_frames: 4,
        animation_speed: 0.15,
    },
];

pub fn get_monster_sprite_config(sprite_type: &str) -> Option<&'static MonsterSpriteConfig> {
    MONSTER_SPRITES.iter().find(|c| c.sprite_type == sprite_type)
}

// =====================================================
// Experience and Gold Reward Helpers
// =====================================================

/// Get monsters suitable for a player of given level (within +/- 5 levels)
pub fn get_monsters_for_level(player_level: i32) -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter()
        .filter(|m| {
            let diff = (m.level - player_level).abs();
            diff <= 5
        })
        .copied()
        .collect()
}

/// Get monsters by minimum level (for dungeon/area filtering)
pub fn get_monsters_by_min_level(min_level: i32) -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter()
        .filter(|m| m.level >= min_level)
        .copied()
        .collect()
}

/// Calculate actual exp reward based on level difference
/// - Same level: 100% exp
/// - Higher level monster: +10% per level (capped at 150%)
/// - Lower level monster: -10% per level (minimum 10%)
pub fn calculate_exp_reward(monster: &MonsterDef, player_level: i32) -> i32 {
    let level_diff = monster.level - player_level;
    
    let multiplier = if level_diff >= 0 {
        // Monster is same level or higher
        (1.0 + (level_diff as f64 * 0.1)).min(1.5)
    } else {
        // Monster is lower level
        (1.0 + (level_diff as f64 * 0.1)).max(0.1)
    };
    
    (monster.exp_reward as f64 * multiplier) as i32
}

/// Calculate actual gold reward (random within range)
pub fn calculate_gold_reward(monster: &MonsterDef) -> i32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    if monster.gold_max > monster.gold_min {
        rng.gen_range(monster.gold_min..=monster.gold_max)
    } else {
        monster.gold_min
    }
}

/// Get total exp needed to kill N monsters of a specific type
pub fn exp_from_monsters(monster_id: i32, count: i32) -> i32 {
    get_monster_by_id(monster_id)
        .map(|m| m.exp_reward * count)
        .unwrap_or(0)
}

/// Estimate how many monsters needed to level up
pub fn monsters_to_level_up(monster: &MonsterDef, current_exp: i64, exp_to_next: i64) -> i32 {
    let exp_needed = exp_to_next - current_exp;
    if exp_needed <= 0 { return 0; }
    
    ((exp_needed as f64) / (monster.exp_reward as f64)).ceil() as i32
}
