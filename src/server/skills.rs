use axum::{Json, extract::Query};
use serde::Deserialize;
use crate::shared::domain::skill::models::Skill;
use crate::server::db::get_db_pool;

#[derive(Deserialize)]
pub struct SkillFilter {
    pub class_id: Option<i32>,
    pub level: Option<i32>,
}

/// Handler to get all skills or filter by class/level
pub async fn get_skills(Query(filter): Query<SkillFilter>) -> Json<Vec<Skill>> {
    let pool = get_db_pool();
    
    let result = match (filter.class_id, filter.level) {
        (Some(cid), Some(lvl)) => {
            sqlx::query_as::<_, Skill>(
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE (class_id = $1 OR class_id IS NULL) AND req_level <= $2"
            )
            .bind(cid)
            .bind(lvl)
            .fetch_all(pool)
            .await
        },
        (Some(cid), None) => {
            sqlx::query_as::<_, Skill>(
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE class_id = $1 OR class_id IS NULL"
            )
            .bind(cid)
            .fetch_all(pool)
            .await
        },
        (None, Some(lvl)) => {
            sqlx::query_as::<_, Skill>(
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE req_level <= $1"
            )
            .bind(lvl)
            .fetch_all(pool)
            .await
        },
        (None, None) => {
            sqlx::query_as::<_, Skill>(
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions"
            )
            .fetch_all(pool)
            .await
        }
    };

    let skills = result.unwrap_or_default();
    Json(skills)
}
