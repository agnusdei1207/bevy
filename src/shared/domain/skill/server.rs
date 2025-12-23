//! Skill API - Shared between client and server
//!
//! CSR: Fetches from REST API
//! SSR: Fetches directly from DB

use crate::shared::domain::skill::models::Skill;

/// Get skills for a class and level
/// On server: queries DB
/// On client: calls API
pub async fn get_skills(class_id: Option<i32>, level: i32) -> Result<Vec<Skill>, String> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "server")] {
            let pool = crate::server::db::get_db_pool();
            
            let skills = sqlx::query_as!(
                Skill,
                "SELECT id, name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value, icon_path FROM skill_definitions WHERE (class_id = $1 OR class_id IS NULL) AND req_level <= $2",
                class_id,
                level
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;
            
            Ok(skills)
        } else {
            // Client side fetch using reqwest
            let base_url = if cfg!(target_arch = "wasm32") {
                "/api/skills"
            } else {
                "http://localhost:3000/api/skills"
            };
            
            let mut query_params = format!("?level={}", level);
            if let Some(cid) = class_id {
                query_params.push_str(&format!("&class_id={}", cid));
            }
            
            let url = format!("{}{}", base_url, query_params);
            
            let response = reqwest::get(url)
                .await
                .map_err(|e| e.to_string())?;
                
            let skills = response
                .json::<Vec<Skill>>()
                .await
                .map_err(|e| e.to_string())?;
                
            Ok(skills)
        }
    }
}
