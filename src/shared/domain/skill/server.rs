//! Skill API - Client fetches from REST API, Server handles DB
//!
//! CSR: Uses mock data or HTTP fetch
//! SSR: Provides handlers for API

use crate::shared::domain::skill::models::Skill;

/// Get skills for a class and level
/// In CSR mode, returns mock data
/// In a real scenario, this would be an HTTP call to /api/skills
pub async fn get_skills(_class: String, level: i32) -> Result<Vec<Skill>, String> {
    // Mock data for CSR - replace with actual API call later
    let all_skills = vec![
        Skill {
            id: 1,
            name: "기본 공격".to_string(),
            description: Some("기본적인 공격 스킬".to_string()),
            level_req: 1,
            class_req: None,
            mp_cost: 0,
            cooldown: 1,
            damage_min: 10,
            damage_max: 15,
            icon_path: None,
        },
        Skill {
            id: 2,
            name: "강타".to_string(),
            description: Some("강력한 일격".to_string()),
            level_req: 3,
            class_req: Some("Warrior".to_string()),
            mp_cost: 10,
            cooldown: 3,
            damage_min: 25,
            damage_max: 35,
            icon_path: None,
        },
        Skill {
            id: 3,
            name: "회전 베기".to_string(),
            description: Some("주변 적을 공격".to_string()),
            level_req: 5,
            class_req: Some("Warrior".to_string()),
            mp_cost: 20,
            cooldown: 5,
            damage_min: 35,
            damage_max: 50,
            icon_path: None,
        },
        Skill {
            id: 4,
            name: "파이어볼".to_string(),
            description: Some("불덩이를 발사".to_string()),
            level_req: 3,
            class_req: Some("Mage".to_string()),
            mp_cost: 15,
            cooldown: 2,
            damage_min: 30,
            damage_max: 45,
            icon_path: None,
        },
    ];
    
    // Filter by level
    let skills: Vec<Skill> = all_skills
        .into_iter()
        .filter(|s| s.level_req <= level)
        .collect();
    
    Ok(skills)
}
