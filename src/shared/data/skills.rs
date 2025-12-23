//! Skill data constants
//!
//! All skill definitions that were previously stored in the database.

/// Effect types for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEffectType {
    Damage,
    Heal,
    Buff,
    Debuff,
}

/// Skill definition
#[derive(Debug, Clone)]
pub struct SkillDef {
    pub id: i32,
    pub name: &'static str,
    pub name_key: &'static str,
    pub class_id: Option<i32>,  // None = available to all
    pub req_level: i32,
    pub mp_cost: i32,
    pub cooldown_ms: i32,
    pub description_key: &'static str,
    pub effect_type: SkillEffectType,
    pub base_value: i32,
    pub icon_path: &'static str,
}

// ============ Warrior Skills ============

pub const BASH: SkillDef = SkillDef {
    id: 1,
    name: "Bash",
    name_key: "skill.bash",
    class_id: Some(1),
    req_level: 1,
    mp_cost: 10,
    cooldown_ms: 1000,
    description_key: "skill.bash.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 50,
    icon_path: "/assets/skills/bash.png",
};

pub const CRASH: SkillDef = SkillDef {
    id: 2,
    name: "Crash",
    name_key: "skill.crash",
    class_id: Some(1),
    req_level: 10,
    mp_cost: 30,
    cooldown_ms: 3000,
    description_key: "skill.crash.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 150,
    icon_path: "/assets/skills/crash.png",
};

pub const IRON_WILL: SkillDef = SkillDef {
    id: 3,
    name: "Iron Will",
    name_key: "skill.iron_will",
    class_id: Some(1),
    req_level: 20,
    mp_cost: 50,
    cooldown_ms: 60000,
    description_key: "skill.iron_will.desc",
    effect_type: SkillEffectType::Buff,
    base_value: 20,
    icon_path: "/assets/skills/iron_will.png",
};

// ============ Rogue Skills ============

pub const DOUBLE_STAB: SkillDef = SkillDef {
    id: 4,
    name: "Double Stab",
    name_key: "skill.double_stab",
    class_id: Some(2),
    req_level: 1,
    mp_cost: 10,
    cooldown_ms: 1000,
    description_key: "skill.double_stab.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 40,
    icon_path: "/assets/skills/double_stab.png",
};

pub const AMBUSH: SkillDef = SkillDef {
    id: 5,
    name: "Ambush",
    name_key: "skill.ambush",
    class_id: Some(2),
    req_level: 10,
    mp_cost: 30,
    cooldown_ms: 5000,
    description_key: "skill.ambush.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 180,
    icon_path: "/assets/skills/ambush.png",
};

// ============ Mage Skills ============

pub const FIREBALL: SkillDef = SkillDef {
    id: 6,
    name: "Fireball",
    name_key: "skill.fireball",
    class_id: Some(3),
    req_level: 1,
    mp_cost: 15,
    cooldown_ms: 1500,
    description_key: "skill.fireball.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 60,
    icon_path: "/assets/skills/fireball.png",
};

pub const THUNDER_BOLT: SkillDef = SkillDef {
    id: 7,
    name: "Thunder Bolt",
    name_key: "skill.thunder_bolt",
    class_id: Some(3),
    req_level: 10,
    mp_cost: 40,
    cooldown_ms: 4000,
    description_key: "skill.thunder_bolt.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 160,
    icon_path: "/assets/skills/thunder_bolt.png",
};

pub const ICE_SHIELD: SkillDef = SkillDef {
    id: 8,
    name: "Ice Shield",
    name_key: "skill.ice_shield",
    class_id: Some(3),
    req_level: 20,
    mp_cost: 60,
    cooldown_ms: 45000,
    description_key: "skill.ice_shield.desc",
    effect_type: SkillEffectType::Buff,
    base_value: 50,
    icon_path: "/assets/skills/ice_shield.png",
};

// ============ Cleric Skills ============

pub const HEAL: SkillDef = SkillDef {
    id: 9,
    name: "Heal",
    name_key: "skill.heal",
    class_id: Some(4),
    req_level: 1,
    mp_cost: 20,
    cooldown_ms: 2000,
    description_key: "skill.heal.desc",
    effect_type: SkillEffectType::Heal,
    base_value: 40,
    icon_path: "/assets/skills/heal.png",
};

pub const HOLY_BOLT: SkillDef = SkillDef {
    id: 10,
    name: "Holy Bolt",
    name_key: "skill.holy_bolt",
    class_id: Some(4),
    req_level: 5,
    mp_cost: 25,
    cooldown_ms: 2000,
    description_key: "skill.holy_bolt.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 50,
    icon_path: "/assets/skills/holy_bolt.png",
};

pub const GREAT_HEAL: SkillDef = SkillDef {
    id: 11,
    name: "Great Heal",
    name_key: "skill.great_heal",
    class_id: Some(4),
    req_level: 20,
    mp_cost: 60,
    cooldown_ms: 5000,
    description_key: "skill.great_heal.desc",
    effect_type: SkillEffectType::Heal,
    base_value: 150,
    icon_path: "/assets/skills/great_heal.png",
};

// ============ Martial Artist Skills ============

pub const PUNCH: SkillDef = SkillDef {
    id: 12,
    name: "Punch",
    name_key: "skill.punch",
    class_id: Some(5),
    req_level: 1,
    mp_cost: 5,
    cooldown_ms: 500,
    description_key: "skill.punch.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 30,
    icon_path: "/assets/skills/punch.png",
};

pub const POWER_KICK: SkillDef = SkillDef {
    id: 13,
    name: "Power Kick",
    name_key: "skill.power_kick",
    class_id: Some(5),
    req_level: 10,
    mp_cost: 20,
    cooldown_ms: 2000,
    description_key: "skill.power_kick.desc",
    effect_type: SkillEffectType::Damage,
    base_value: 80,
    icon_path: "/assets/skills/power_kick.png",
};

/// All skill definitions
pub const ALL_SKILLS: &[&SkillDef] = &[
    &BASH,
    &CRASH,
    &IRON_WILL,
    &DOUBLE_STAB,
    &AMBUSH,
    &FIREBALL,
    &THUNDER_BOLT,
    &ICE_SHIELD,
    &HEAL,
    &HOLY_BOLT,
    &GREAT_HEAL,
    &PUNCH,
    &POWER_KICK,
];

/// Get skill by ID
pub fn get_skill_by_id(id: i32) -> Option<&'static SkillDef> {
    ALL_SKILLS.iter().find(|s| s.id == id).copied()
}

/// Get skills for a specific class
pub fn get_skills_for_class(class_id: i32) -> Vec<&'static SkillDef> {
    ALL_SKILLS.iter()
        .filter(|s| s.class_id == Some(class_id) || s.class_id.is_none())
        .copied()
        .collect()
}

/// Get available skills for a class at a given level
pub fn get_available_skills(class_id: i32, level: i32) -> Vec<&'static SkillDef> {
    ALL_SKILLS.iter()
        .filter(|s| {
            (s.class_id == Some(class_id) || s.class_id.is_none()) && s.req_level <= level
        })
        .copied()
        .collect()
}
