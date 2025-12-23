use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::shared::domain::skill::models::Skill;
use crate::shared::data::skills::{ALL_SKILLS, SkillEffectType};

#[derive(Deserialize)]
pub struct SkillFilter {
    pub class_id: Option<i32>,
    pub level: Option<i32>,
}

/// Convert SkillDef to Skill domain model
fn skill_def_to_skill(def: &crate::shared::data::skills::SkillDef) -> Skill {
    Skill {
        id: def.id,
        name: def.name.to_string(),
        class_id: def.class_id,
        req_level: def.req_level,
        mp_cost: def.mp_cost,
        cooldown_ms: def.cooldown_ms,
        description: Some(def.description_key.to_string()),
        effect_type: Some(match def.effect_type {
            SkillEffectType::Damage => "damage",
            SkillEffectType::Heal => "heal",
            SkillEffectType::Buff => "buff",
            SkillEffectType::Debuff => "debuff",
        }.to_string()),
        base_value: def.base_value,
        icon_path: Some(def.icon_path.to_string()),
    }
}

/// Handler to get all skills or filter by class/level
/// Now uses Rust constants for perfect consistency with client.
pub async fn get_skills(Query(filter): Query<SkillFilter>) -> Json<Vec<Skill>> {
    let mut skills: Vec<Skill> = ALL_SKILLS.iter()
        .map(|def| skill_def_to_skill(def))
        .collect();

    if let Some(cid) = filter.class_id {
        skills.retain(|s| s.class_id == Some(cid) || s.class_id.is_none());
    }

    if let Some(lvl) = filter.level {
        skills.retain(|s| s.req_level <= lvl);
    }

    Json(skills)
}
