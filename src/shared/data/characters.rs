//! Character and class data constants
//!
//! All character class definitions and sprite configurations.

use crate::shared::domain::shared::models::Stats;

/// Player class definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassId {
    Warrior = 1,
    Rogue = 2,
    Mage = 3,
    Cleric = 4,
    MartialArtist = 5,
}

/// Class definition with all metadata
#[derive(Debug, Clone)]
pub struct ClassDef {
    pub id: ClassId,
    pub name: &'static str,
    pub name_key: &'static str,
    pub description_key: &'static str,
    pub base_stats: Stats,
    pub hp_per_level: i32,
    pub mp_per_level: i32,
    pub stat_points_per_level: i32,
    pub primary_stat: &'static str,
    pub weapon_type: &'static str,
}

pub const WARRIOR: ClassDef = ClassDef {
    id: ClassId::Warrior,
    name: "Warrior",
    name_key: "class.warrior",
    description_key: "class.warrior.desc",
    base_stats: Stats { str_stat: 10, dex_stat: 5, int_stat: 3, wis_stat: 3, con_stat: 10 },
    hp_per_level: 25,
    mp_per_level: 5,
    stat_points_per_level: 2,
    primary_stat: "str_stat",
    weapon_type: "sword",
};

pub const ROGUE: ClassDef = ClassDef {
    id: ClassId::Rogue,
    name: "Rogue",
    name_key: "class.rogue",
    description_key: "class.rogue.desc",
    base_stats: Stats { str_stat: 7, dex_stat: 10, int_stat: 3, wis_stat: 5, con_stat: 5 },
    hp_per_level: 18,
    mp_per_level: 8,
    stat_points_per_level: 2,
    primary_stat: "dex_stat",
    weapon_type: "dagger",
};

pub const MAGE: ClassDef = ClassDef {
    id: ClassId::Mage,
    name: "Mage",
    name_key: "class.mage",
    description_key: "class.mage.desc",
    base_stats: Stats { str_stat: 3, dex_stat: 4, int_stat: 10, wis_stat: 10, con_stat: 3 },
    hp_per_level: 12,
    mp_per_level: 20,
    stat_points_per_level: 2,
    primary_stat: "int_stat",
    weapon_type: "staff",
};

pub const CLERIC: ClassDef = ClassDef {
    id: ClassId::Cleric,
    name: "Cleric",
    name_key: "class.cleric",
    description_key: "class.cleric.desc",
    base_stats: Stats { str_stat: 4, dex_stat: 4, int_stat: 7, wis_stat: 10, con_stat: 5 },
    hp_per_level: 18,
    mp_per_level: 15,
    stat_points_per_level: 2,
    primary_stat: "wis_stat",
    weapon_type: "mace",
};

pub const MARTIAL_ARTIST: ClassDef = ClassDef {
    id: ClassId::MartialArtist,
    name: "Martial Artist",
    name_key: "class.martial_artist",
    description_key: "class.martial_artist.desc",
    base_stats: Stats { str_stat: 8, dex_stat: 8, int_stat: 3, wis_stat: 3, con_stat: 8 },
    hp_per_level: 22,
    mp_per_level: 8,
    stat_points_per_level: 2,
    primary_stat: "str_stat",
    weapon_type: "knuckle",
};

/// All class definitions
pub const ALL_CLASSES: &[&ClassDef] = &[
    &WARRIOR,
    &ROGUE,
    &MAGE,
    &CLERIC,
    &MARTIAL_ARTIST,
];

/// Get class by ID
pub fn get_class_by_id(id: i32) -> Option<&'static ClassDef> {
    match id {
        1 => Some(&WARRIOR),
        2 => Some(&ROGUE),
        3 => Some(&MAGE),
        4 => Some(&CLERIC),
        5 => Some(&MARTIAL_ARTIST),
        _ => None,
    }
}

/// Character sprite configuration
#[derive(Debug, Clone)]
pub struct CharacterSpriteConfig {
    pub class_name: &'static str,
    pub gender: &'static str,
    pub sprite_path: &'static str,
    pub frame_width: u32,
    pub frame_height: u32,
    pub idle_frames: u32,
    pub walk_frames: u32,
    pub attack_frames: u32,
    pub cast_frames: u32,
    pub death_frames: u32,
}

/// Character sprite paths and configurations
pub const CHARACTER_SPRITES: &[CharacterSpriteConfig] = &[
    // Warrior
    CharacterSpriteConfig {
        class_name: "warrior",
        gender: "male",
        sprite_path: "/assets/characters/warrior/male_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 6,
        cast_frames: 0,
        death_frames: 4,
    },
    CharacterSpriteConfig {
        class_name: "warrior",
        gender: "female",
        sprite_path: "/assets/characters/warrior/female_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 6,
        cast_frames: 0,
        death_frames: 4,
    },
    // Rogue
    CharacterSpriteConfig {
        class_name: "rogue",
        gender: "male",
        sprite_path: "/assets/characters/rogue/male_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 0,
        death_frames: 4,
    },
    CharacterSpriteConfig {
        class_name: "rogue",
        gender: "female",
        sprite_path: "/assets/characters/rogue/female_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 0,
        death_frames: 4,
    },
    // Mage
    CharacterSpriteConfig {
        class_name: "mage",
        gender: "male",
        sprite_path: "/assets/characters/mage/male_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 6,
        death_frames: 4,
    },
    CharacterSpriteConfig {
        class_name: "mage",
        gender: "female",
        sprite_path: "/assets/characters/mage/female_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 6,
        death_frames: 4,
    },
    // Cleric
    CharacterSpriteConfig {
        class_name: "cleric",
        gender: "male",
        sprite_path: "/assets/characters/cleric/male_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 6,
        death_frames: 4,
    },
    CharacterSpriteConfig {
        class_name: "cleric",
        gender: "female",
        sprite_path: "/assets/characters/cleric/female_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 4,
        cast_frames: 6,
        death_frames: 4,
    },
    // Martial Artist
    CharacterSpriteConfig {
        class_name: "martial_artist",
        gender: "male",
        sprite_path: "/assets/characters/martial_artist/male_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 6,
        cast_frames: 0,
        death_frames: 4,
    },
    CharacterSpriteConfig {
        class_name: "martial_artist",
        gender: "female",
        sprite_path: "/assets/characters/martial_artist/female_spritesheet.png",
        frame_width: 48,
        frame_height: 64,
        idle_frames: 4,
        walk_frames: 8,
        attack_frames: 6,
        cast_frames: 0,
        death_frames: 4,
    },
];

/// Get character sprite config
pub fn get_character_sprite(class_name: &str, gender: &str) -> Option<&'static CharacterSpriteConfig> {
    CHARACTER_SPRITES.iter()
        .find(|c| c.class_name == class_name && c.gender == gender)
}

// =====================================================
// Experience System (Level 1-99)
// =====================================================

/// Maximum level
pub const MAX_LEVEL: i32 = 99;

/// Experience entry for a level
#[derive(Debug, Clone, Copy)]
pub struct ExpEntry {
    pub level: i32,
    pub exp_to_next: i64,      // 다음 레벨까지 필요한 경험치
    pub total_exp: i64,        // 이 레벨에 도달하기까지 필요한 총 경험치
}

/// Complete Experience Table (Level 1-99)
/// Formula: exp_to_next = floor(100 * level^1.5)
pub const EXP_TABLE: [ExpEntry; 99] = [
    ExpEntry { level: 1, exp_to_next: 100, total_exp: 0 },
    ExpEntry { level: 2, exp_to_next: 283, total_exp: 100 },
    ExpEntry { level: 3, exp_to_next: 520, total_exp: 383 },
    ExpEntry { level: 4, exp_to_next: 800, total_exp: 903 },
    ExpEntry { level: 5, exp_to_next: 1118, total_exp: 1703 },
    ExpEntry { level: 6, exp_to_next: 1470, total_exp: 2821 },
    ExpEntry { level: 7, exp_to_next: 1852, total_exp: 4291 },
    ExpEntry { level: 8, exp_to_next: 2263, total_exp: 6143 },
    ExpEntry { level: 9, exp_to_next: 2700, total_exp: 8406 },
    ExpEntry { level: 10, exp_to_next: 3162, total_exp: 11106 },
    ExpEntry { level: 11, exp_to_next: 3648, total_exp: 14268 },
    ExpEntry { level: 12, exp_to_next: 4157, total_exp: 17916 },
    ExpEntry { level: 13, exp_to_next: 4686, total_exp: 22073 },
    ExpEntry { level: 14, exp_to_next: 5237, total_exp: 26759 },
    ExpEntry { level: 15, exp_to_next: 5809, total_exp: 31996 },
    ExpEntry { level: 16, exp_to_next: 6400, total_exp: 37805 },
    ExpEntry { level: 17, exp_to_next: 7010, total_exp: 44205 },
    ExpEntry { level: 18, exp_to_next: 7638, total_exp: 51215 },
    ExpEntry { level: 19, exp_to_next: 8283, total_exp: 58853 },
    ExpEntry { level: 20, exp_to_next: 8944, total_exp: 67136 },
    ExpEntry { level: 21, exp_to_next: 9621, total_exp: 76080 },
    ExpEntry { level: 22, exp_to_next: 10314, total_exp: 85701 },
    ExpEntry { level: 23, exp_to_next: 11021, total_exp: 96015 },
    ExpEntry { level: 24, exp_to_next: 11743, total_exp: 107036 },
    ExpEntry { level: 25, exp_to_next: 12480, total_exp: 118779 },
    ExpEntry { level: 26, exp_to_next: 13230, total_exp: 131259 },
    ExpEntry { level: 27, exp_to_next: 13993, total_exp: 144489 },
    ExpEntry { level: 28, exp_to_next: 14770, total_exp: 158482 },
    ExpEntry { level: 29, exp_to_next: 15559, total_exp: 173252 },
    ExpEntry { level: 30, exp_to_next: 16362, total_exp: 188811 },
    ExpEntry { level: 31, exp_to_next: 17177, total_exp: 205173 },
    ExpEntry { level: 32, exp_to_next: 18004, total_exp: 222350 },
    ExpEntry { level: 33, exp_to_next: 18843, total_exp: 240354 },
    ExpEntry { level: 34, exp_to_next: 19694, total_exp: 259197 },
    ExpEntry { level: 35, exp_to_next: 20557, total_exp: 278891 },
    ExpEntry { level: 36, exp_to_next: 21431, total_exp: 299448 },
    ExpEntry { level: 37, exp_to_next: 22316, total_exp: 320879 },
    ExpEntry { level: 38, exp_to_next: 23212, total_exp: 343195 },
    ExpEntry { level: 39, exp_to_next: 24118, total_exp: 366407 },
    ExpEntry { level: 40, exp_to_next: 25035, total_exp: 390525 },
    ExpEntry { level: 41, exp_to_next: 25962, total_exp: 415560 },
    ExpEntry { level: 42, exp_to_next: 26899, total_exp: 441522 },
    ExpEntry { level: 43, exp_to_next: 27846, total_exp: 468421 },
    ExpEntry { level: 44, exp_to_next: 28803, total_exp: 496267 },
    ExpEntry { level: 45, exp_to_next: 29770, total_exp: 525070 },
    ExpEntry { level: 46, exp_to_next: 30746, total_exp: 554840 },
    ExpEntry { level: 47, exp_to_next: 31731, total_exp: 585586 },
    ExpEntry { level: 48, exp_to_next: 32726, total_exp: 617317 },
    ExpEntry { level: 49, exp_to_next: 33729, total_exp: 650043 },
    ExpEntry { level: 50, exp_to_next: 34741, total_exp: 683772 },
    ExpEntry { level: 51, exp_to_next: 35762, total_exp: 718513 },
    ExpEntry { level: 52, exp_to_next: 36791, total_exp: 754275 },
    ExpEntry { level: 53, exp_to_next: 37829, total_exp: 791066 },
    ExpEntry { level: 54, exp_to_next: 38874, total_exp: 828895 },
    ExpEntry { level: 55, exp_to_next: 39928, total_exp: 867769 },
    ExpEntry { level: 56, exp_to_next: 40989, total_exp: 907697 },
    ExpEntry { level: 57, exp_to_next: 42058, total_exp: 948686 },
    ExpEntry { level: 58, exp_to_next: 43135, total_exp: 990744 },
    ExpEntry { level: 59, exp_to_next: 44219, total_exp: 1033879 },
    ExpEntry { level: 60, exp_to_next: 45310, total_exp: 1078098 },
    ExpEntry { level: 61, exp_to_next: 46409, total_exp: 1123408 },
    ExpEntry { level: 62, exp_to_next: 47514, total_exp: 1169817 },
    ExpEntry { level: 63, exp_to_next: 48627, total_exp: 1217331 },
    ExpEntry { level: 64, exp_to_next: 49746, total_exp: 1265958 },
    ExpEntry { level: 65, exp_to_next: 50872, total_exp: 1315704 },
    ExpEntry { level: 66, exp_to_next: 52005, total_exp: 1366576 },
    ExpEntry { level: 67, exp_to_next: 53144, total_exp: 1418581 },
    ExpEntry { level: 68, exp_to_next: 54290, total_exp: 1471725 },
    ExpEntry { level: 69, exp_to_next: 55442, total_exp: 1526015 },
    ExpEntry { level: 70, exp_to_next: 56600, total_exp: 1581457 },
    ExpEntry { level: 71, exp_to_next: 57764, total_exp: 1638057 },
    ExpEntry { level: 72, exp_to_next: 58935, total_exp: 1695821 },
    ExpEntry { level: 73, exp_to_next: 60111, total_exp: 1754756 },
    ExpEntry { level: 74, exp_to_next: 61293, total_exp: 1814867 },
    ExpEntry { level: 75, exp_to_next: 62481, total_exp: 1876160 },
    ExpEntry { level: 76, exp_to_next: 63675, total_exp: 1938641 },
    ExpEntry { level: 77, exp_to_next: 64874, total_exp: 2002316 },
    ExpEntry { level: 78, exp_to_next: 66079, total_exp: 2067190 },
    ExpEntry { level: 79, exp_to_next: 67290, total_exp: 2133269 },
    ExpEntry { level: 80, exp_to_next: 68506, total_exp: 2200559 },
    ExpEntry { level: 81, exp_to_next: 69727, total_exp: 2269065 },
    ExpEntry { level: 82, exp_to_next: 70953, total_exp: 2338792 },
    ExpEntry { level: 83, exp_to_next: 72185, total_exp: 2409745 },
    ExpEntry { level: 84, exp_to_next: 73421, total_exp: 2481930 },
    ExpEntry { level: 85, exp_to_next: 74663, total_exp: 2555351 },
    ExpEntry { level: 86, exp_to_next: 75909, total_exp: 2630014 },
    ExpEntry { level: 87, exp_to_next: 77161, total_exp: 2705923 },
    ExpEntry { level: 88, exp_to_next: 78417, total_exp: 2783084 },
    ExpEntry { level: 89, exp_to_next: 79678, total_exp: 2861501 },
    ExpEntry { level: 90, exp_to_next: 80944, total_exp: 2941179 },
    ExpEntry { level: 91, exp_to_next: 82215, total_exp: 3022123 },
    ExpEntry { level: 92, exp_to_next: 83490, total_exp: 3104338 },
    ExpEntry { level: 93, exp_to_next: 84770, total_exp: 3187828 },
    ExpEntry { level: 94, exp_to_next: 86054, total_exp: 3272598 },
    ExpEntry { level: 95, exp_to_next: 87343, total_exp: 3358652 },
    ExpEntry { level: 96, exp_to_next: 88636, total_exp: 3445995 },
    ExpEntry { level: 97, exp_to_next: 89933, total_exp: 3534631 },
    ExpEntry { level: 98, exp_to_next: 91235, total_exp: 3624564 },
    ExpEntry { level: 99, exp_to_next: 0, total_exp: 3715799 },  // Max level
];

/// Get exp entry for a specific level (1-99)
pub fn get_exp_entry(level: i32) -> Option<&'static ExpEntry> {
    if level >= 1 && level <= 99 {
        Some(&EXP_TABLE[(level - 1) as usize])
    } else {
        None
    }
}

/// Get exp needed to reach next level
pub fn exp_to_next_level(level: i32) -> i64 {
    get_exp_entry(level).map(|e| e.exp_to_next).unwrap_or(0)
}

/// Get total exp needed to reach a level from level 1
pub fn total_exp_for_level(level: i32) -> i64 {
    get_exp_entry(level).map(|e| e.total_exp).unwrap_or(0)
}

/// Calculate level from total accumulated exp
pub fn level_from_total_exp(total_exp: i64) -> i32 {
    for entry in EXP_TABLE.iter().rev() {
        if total_exp >= entry.total_exp {
            return entry.level;
        }
    }
    1
}

/// Calculate exp progress within current level (0.0 to 1.0)
pub fn exp_progress(level: i32, current_exp: i64) -> f32 {
    let exp_needed = exp_to_next_level(level);
    if exp_needed <= 0 { return 1.0; }  // Max level
    (current_exp as f32 / exp_needed as f32).min(1.0)
}

/// Check if player can level up
pub fn can_level_up(level: i32, current_exp: i64) -> bool {
    level < MAX_LEVEL && current_exp >= exp_to_next_level(level)
}

/// Level-up rewards
#[derive(Debug, Clone)]
pub struct LevelUpReward {
    pub stat_points: i32,
    pub hp_bonus: i32,
    pub mp_bonus: i32,
    pub skill_unlock: Option<i32>,
}

/// Get level-up rewards for a class
pub fn get_level_up_reward(class_id: i32, new_level: i32) -> LevelUpReward {
    let class = get_class_by_id(class_id);
    let (hp_bonus, mp_bonus) = match class {
        Some(c) => (c.hp_per_level, c.mp_per_level),
        None => (20, 10),
    };
    
    let skill_unlock = crate::shared::data::skills::ALL_SKILLS.iter()
        .find(|s| s.class_id == Some(class_id) && s.req_level == new_level)
        .map(|s| s.id);
    
    LevelUpReward {
        stat_points: 2,
        hp_bonus,
        mp_bonus,
        skill_unlock,
    }
}

// =====================================================
// Default starting values
// =====================================================

pub mod defaults {
    pub const STARTING_GOLD: i64 = 100;
    pub const STARTING_MAP: &str = "village_milles";
    pub const STARTING_X: f64 = 400.0;
    pub const STARTING_Y: f64 = 300.0;
    pub const INVENTORY_SLOTS: usize = 24;
    pub const BASE_ATTACK_COOLDOWN: f64 = 1000.0;
    pub const STAT_POINTS_PER_LEVEL: i32 = 2;
}

// =====================================================
// HP/MP Calculation
// =====================================================

/// Calculate max HP for a character
pub fn calculate_max_hp(class_id: i32, level: i32, bonus_con_stat: i32) -> i32 {
    let class = get_class_by_id(class_id);
    let (base_hp, hp_per_level, base_con_stat) = match class {
        Some(c) => (100, c.hp_per_level, c.base_stats.con_stat),
        None => (100, 20, 10),
    };
    
    let total_con_stat = base_con_stat + bonus_con_stat;
    base_hp + (level * hp_per_level) + (total_con_stat * 5)
}

/// Calculate max MP for a character
pub fn calculate_max_mp(class_id: i32, level: i32, bonus_wis_stat: i32) -> i32 {
    let class = get_class_by_id(class_id);
    let (base_mp, mp_per_level, base_wis_stat) = match class {
        Some(c) => (50, c.mp_per_level, c.base_stats.wis_stat),
        None => (50, 10, 10),
    };
    
    let total_wis_stat = base_wis_stat + bonus_wis_stat;
    base_mp + (level * mp_per_level) + (total_wis_stat * 3)
}

/// Calculate attack power
pub fn calculate_attack(class_id: i32, level: i32, bonus_str_stat: i32, weapon_attack: i32) -> (i32, i32) {
    let class = get_class_by_id(class_id);
    let base_str_stat = match class {
        Some(c) => c.base_stats.str_stat,
        None => 10,
    };
    
    let total_str_stat = base_str_stat + bonus_str_stat;
    let base_attack = 5 + level;
    let attack_min = base_attack + (total_str_stat / 2) + weapon_attack;
    let attack_max = attack_min + (level / 2) + (total_str_stat / 4);
    
    (attack_min, attack_max)
}
