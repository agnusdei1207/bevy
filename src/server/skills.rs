use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::shared::domain::skill::models::Skill;
use crate::server::db::get_db;

#[derive(Deserialize)]
pub struct SkillFilter {
    pub class_id: Option<i32>,
    pub level: Option<i32>,
}

/// Handler to get all skills or filter by class/level
pub async fn get_skills(Query(filter): Query<SkillFilter>) -> Json<Vec<Skill>> {
    let pool = get_db();
    
    let skills = match (filter.class_id, filter.level) {
        (Some(cid), Some(lvl)) => {
            sqlx::query_as!(
                Skill,
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE (class_id = $1 OR class_id IS NULL) AND req_level <= $2",
                cid,
                lvl
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        },
        (Some(cid), None) => {
            sqlx::query_as!(
                Skill,
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE class_id = $1 OR class_id IS NULL",
                cid
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        },
        (None, Some(lvl)) => {
            sqlx::query_as!(
                Skill,
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE req_level <= $1",
                lvl
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        },
        (None, None) => {
            sqlx::query_as!(
                Skill,
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions"
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        }
    };

    Json(skills)
}
