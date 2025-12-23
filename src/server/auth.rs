//! Authentication handlers - Axum REST API
//!
//! SSR feature only

#[cfg(feature = "server")]
use axum::{Json, Extension};
#[cfg(feature = "server")]
use sqlx::PgPool;

use serde::{Deserialize, Serialize};
use crate::shared::domain::player::Player;

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
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

// --- SSR Handlers ---

#[cfg(feature = "server")]
pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Json<LoginResponse> {
    use bcrypt::verify;
    use sqlx::Row;
    
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
        SELECT c.id, c.name, c.gender, c.class_id, c.level, c.exp, c.gold, c.map_id, c.x, c.y,
               s.str, s.dex, s.con, s.int_stat, s.wis, s.stat_points_available
        FROM characters c
        LEFT JOIN character_stats s ON c.id = s.character_id
        WHERE c.user_id = $1
        LIMIT 1
        "#
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);
    
    if let Some(c) = char_row {
        use crate::shared::domain::character::models::PlayerClass;
        use crate::shared::domain::shared::models::{Stats, CombatStats, Position, Direction};
        
        let class_id: Option<i32> = c.get("class_id");
        let player_class = match class_id {
            Some(1) => PlayerClass::Warrior,
            Some(2) => PlayerClass::Rogue,
            Some(3) => PlayerClass::Mage,
            Some(4) => PlayerClass::Cleric,
            Some(5) => PlayerClass::MartialArtist,
            _ => PlayerClass::Warrior,
        };
        
        let stats = Stats {
            str: c.try_get::<i32, _>("str").unwrap_or(10),
            dex: c.try_get::<i32, _>("dex").unwrap_or(10),
            con: c.try_get::<i32, _>("con").unwrap_or(10),
            int: c.try_get::<i32, _>("int_stat").unwrap_or(10),
            wis: c.try_get::<i32, _>("wis").unwrap_or(10),
        };
        
        let level: i32 = c.try_get("level").unwrap_or(1);
        let combat_stats = CombatStats::from_stats(&stats, level);

        let char_id: uuid::Uuid = c.get("id");
        let player = Player {
            id: char_id.to_string(),
            username: c.get("name"),
            gender: c.get("gender"),
            class: player_class,
            level,
            exp: c.try_get("exp").unwrap_or(0),
            exp_to_next_level: 100,
            stats,
            stat_points: c.try_get("stat_points_available").unwrap_or(0),
            combat_stats,
            equipment: std::collections::HashMap::new(),
            inventory: vec![None; 24],
            current_map: c.get("map_id"),
            position: Position { 
                x: c.try_get::<i32, _>("x").unwrap_or(400) as f64, 
                y: c.try_get::<i32, _>("y").unwrap_or(300) as f64 
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
    
    // Create Character
    let char_id = Uuid::new_v4();
    if let Err(e) = sqlx::query(
        "INSERT INTO characters (id, user_id, name, class_id, gender) VALUES ($1, $2, $3, $4, 'male')"
    )
    .bind(char_id)
    .bind(user_id)
    .bind(&req.username)
    .bind(req.class_idx)
    .execute(&pool)
    .await {
        return Json(RegisterResponse {
            success: false,
            message: format!("Failed to create character: {}", e),
        });
    }
    
    // Create Stats
    if let Err(e) = sqlx::query(
        "INSERT INTO character_stats (character_id, str, dex, con, int_stat, wis) VALUES ($1, 10, 10, 10, 10, 10)"
    )
    .bind(char_id)
    .execute(&pool)
    .await {
        return Json(RegisterResponse {
            success: false,
            message: format!("Failed to create stats: {}", e),
        });
    }
    
    Json(RegisterResponse {
        success: true,
        message: "Registration successful".to_string(),
    })
}
