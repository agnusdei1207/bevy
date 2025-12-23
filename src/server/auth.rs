//! Authentication handlers - Axum REST API
//!
//! Server feature only

#[cfg(feature = "server")]
use axum::{Json, Extension};
#[cfg(feature = "server")]
use sqlx::PgPool;

use serde::{Deserialize, Serialize};
use crate::shared::domain::Player;

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub player: Option<Player>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub class_idx: i32,
    pub gender: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

// --- Server Handlers ---

#[cfg(feature = "server")]
pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Json<LoginResponse> {
    use bcrypt::verify;
    use sqlx::Row;
    use crate::shared::domain::character::models::PlayerClass;
    use crate::shared::domain::shared::models::{Stats, CombatStats, Position, Direction};
    use crate::shared::data::characters::get_class_by_id;
    
    // 1. Get User
    let row: Option<(uuid::Uuid, String)> = sqlx::query_as(
        "SELECT id, password_hash FROM users WHERE username = $1"
    )
    .bind(&req.username)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);
    
    let (user_id, password_hash) = match row {
        Some(u) => u,
        None => return Json(LoginResponse {
            success: false,
            player: None,
            error: Some("User not found".to_string()),
        }),
    };
    
    // 2. Verify Password
    if !verify(&req.password, &password_hash).unwrap_or(false) {
        return Json(LoginResponse {
            success: false,
            player: None,
            error: Some("Invalid password".to_string()),
        });
    }
    
    // 3. Get Character
    let char_row = sqlx::query(
        r#"
        SELECT id, name, gender, class_id, level, exp, gold, current_map, pos_x, pos_y,
               bonus_str_stat, bonus_dex_stat, bonus_int_stat, bonus_wis_stat, bonus_con_stat, stat_points
        FROM characters
        WHERE user_id = $1
        LIMIT 1
        "#
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);
    
    if let Some(c) = char_row {
        let class_id: i32 = c.get("class_id");
        let player_class = match class_id {
            1 => PlayerClass::Warrior,
            2 => PlayerClass::Rogue,
            3 => PlayerClass::Mage,
            4 => PlayerClass::Cleric,
            5 => PlayerClass::MartialArtist,
            _ => PlayerClass::Warrior,
        };
        
        let bonus_stats = Stats {
            str_stat: c.try_get::<i32, _>("bonus_str_stat").unwrap_or(0),
            dex_stat: c.try_get::<i32, _>("bonus_dex_stat").unwrap_or(0),
            int_stat: c.try_get::<i32, _>("bonus_int_stat").unwrap_or(0),
            wis_stat: c.try_get::<i32, _>("bonus_wis_stat").unwrap_or(0),
            con_stat: c.try_get::<i32, _>("bonus_con_stat").unwrap_or(0),
        };
        
        // Load base stats from class definition
        let class_def = get_class_by_id(class_id).expect("Invalid class ID in DB");
        let final_stats = class_def.base_stats + bonus_stats;
        
        let level: i32 = c.try_get("level").unwrap_or(1);
        let combat_stats = CombatStats::from_stats(&final_stats, level);

        let char_id: uuid::Uuid = c.get("id");
        let player = Player {
            id: char_id.to_string(),
            username: c.get("name"),
            gender: c.get("gender"),
            class: player_class,
            level,
            exp: c.try_get("exp").unwrap_or(0),
            exp_to_next_level: crate::shared::data::characters::exp_to_next_level(level),
            stats: final_stats,
            stat_points: c.try_get("stat_points").unwrap_or(0),
            combat_stats,
            equipment: std::collections::HashMap::new(),
            inventory: vec![None; 24],
            current_map: c.get("current_map"),
            position: Position { 
                x: c.try_get::<f64, _>("pos_x").unwrap_or(400.0), 
                y: c.try_get::<f64, _>("pos_y").unwrap_or(300.0) 
            },
            direction: Direction::Down,
            gold: c.try_get("gold").unwrap_or(0),
            is_moving: false,
            is_attacking: false,
            target_monster_id: None,
            last_attack_time: 0.0,
            attack_cooldown: 1000.0,
        };
        
        return Json(LoginResponse {
            success: true,
            player: Some(player),
            error: None,
        });
    }
    
    Json(LoginResponse {
        success: false,
        player: None,
        error: Some("Character not found".to_string()),
    })
}

#[cfg(feature = "server")]
pub async fn register_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    use bcrypt::{hash, DEFAULT_COST};
    use uuid::Uuid;
    
    // Hash password
    let hashed = match hash(&req.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Json(RegisterResponse {
            success: false,
            message: "Failed to hash password".to_string(),
        }),
    };
    
    // Create User
    let user_id = Uuid::new_v4();
    if let Err(e) = sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind(&req.username)
    .bind(&hashed)
    .execute(&pool)
    .await {
        return Json(RegisterResponse {
            success: false,
            message: format!("Failed to create user: {}", e),
        });
    }
    
    // Create Character with default starting values
    let char_id = Uuid::new_v4();
    if let Err(e) = sqlx::query(
        "INSERT INTO characters (id, user_id, name, class_id, gender) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(char_id)
    .bind(user_id)
    .bind(&req.username)
    .bind(req.class_idx)
    .bind(&req.gender)
    .execute(&pool)
    .await {
        return Json(RegisterResponse {
            success: false,
            message: format!("Failed to create character: {}", e),
        });
    }
    
    Json(RegisterResponse {
        success: true,
        message: "Registration successful".to_string(),
    })
}
