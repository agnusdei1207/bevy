//! Monster API handlers
//!
//! Now returning data from Rust constants for consistency.

#[cfg(feature = "server")]
use axum::response::Json;
use crate::shared::domain::monster::{MonsterDataDto, LootDrop};
use crate::shared::data::monsters::{ALL_MONSTERS, get_monster_drops};

/// Get all monster definitions from constants
pub async fn get_monsters() -> Json<Vec<MonsterDataDto>> {
    let dtos = ALL_MONSTERS.iter()
        .map(|m| {
            let drops = get_monster_drops(m.id);
            let loot_table: Vec<LootDrop> = drops.iter()
                .map(|d| LootDrop {
                    item_id: d.item_id,
                    probability: d.probability,
                    min_quantity: d.min_quantity,
                    max_quantity: d.max_quantity,
                })
                .collect();
            
            MonsterDataDto {
                id: m.id,
                name: m.name.to_string(),
                level: m.level,
                hp_max: m.hp_max,
                mp_max: m.mp_max,
                attack_min: m.attack_min,
                attack_max: m.attack_max,
                defense: m.defense,
                exp_reward: m.exp_reward,
                gold_min: m.gold_min,
                gold_max: m.gold_max,
                sprite_path: Some(m.sprite_path()),
                ai_type: Some(match m.ai_type {
                    crate::shared::domain::monster::MonsterAIType::Passive => "passive",
                    crate::shared::domain::monster::MonsterAIType::Aggressive => "aggressive",
                    crate::shared::domain::monster::MonsterAIType::Defensive => "defensive",
                }.to_string()),
                sprite_type: Some(m.sprite_type.to_string()),
                sprite_size: Some(match m.sprite_size {
                    crate::shared::domain::monster::SpriteSize::Small => "small",
                    crate::shared::domain::monster::SpriteSize::Medium => "medium",
                    crate::shared::domain::monster::SpriteSize::Large => "large",
                    crate::shared::domain::monster::SpriteSize::Boss => "boss",
                }.to_string()),
                detection_range: Some(m.detection_range),
                attack_range: Some(m.attack_range),
                move_speed: Some(m.move_speed),
                description: Some(m.description_key.to_string()),
                metadata: None,
                loot: if loot_table.is_empty() { None } else { Some(loot_table) },
            }
        })
        .collect();

    Json(dtos)
}


/// Get monster by ID from constants
pub async fn get_monster_by_id(
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<Option<MonsterDataDto>> {
    let monster = ALL_MONSTERS.iter().find(|m| m.id == id);
    
    let dto = monster.map(|m| {
        let drops = get_monster_drops(m.id);
        let loot_table: Vec<LootDrop> = drops.iter()
            .map(|d| LootDrop {
                item_id: d.item_id,
                probability: d.probability,
                min_quantity: d.min_quantity,
                max_quantity: d.max_quantity,
            })
            .collect();
        
        MonsterDataDto {
            id: m.id,
            name: m.name.to_string(),
            level: m.level,
            hp_max: m.hp_max,
            mp_max: m.mp_max,
            attack_min: m.attack_min,
            attack_max: m.attack_max,
            defense: m.defense,
            exp_reward: m.exp_reward,
            gold_min: m.gold_min,
            gold_max: m.gold_max,
            sprite_path: Some(m.sprite_path()),
            ai_type: Some(match m.ai_type {
                crate::shared::domain::monster::MonsterAIType::Passive => "passive",
                crate::shared::domain::monster::MonsterAIType::Aggressive => "aggressive",
                crate::shared::domain::monster::MonsterAIType::Defensive => "defensive",
            }.to_string()),
            sprite_type: Some(m.sprite_type.to_string()),
            sprite_size: Some(match m.sprite_size {
                crate::shared::domain::monster::SpriteSize::Small => "small",
                crate::shared::domain::monster::SpriteSize::Medium => "medium",
                crate::shared::domain::monster::SpriteSize::Large => "large",
                crate::shared::domain::monster::SpriteSize::Boss => "boss",
            }.to_string()),
            detection_range: Some(m.detection_range),
            attack_range: Some(m.attack_range),
            move_speed: Some(m.move_speed),
            description: Some(m.description_key.to_string()),
            metadata: None,
            loot: if loot_table.is_empty() { None } else { Some(loot_table) },
        }
    });

    Json(dto)
}
