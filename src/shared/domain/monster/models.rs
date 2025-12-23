use serde::{Deserialize, Serialize};
use crate::shared::domain::shared::models::{Position, Direction};

/// Sprite size for monsters - determines frame dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SpriteSize {
    Small,   // 32x32 (Lv 1-10)
    Medium,  // 48x48 (Lv 11-50)
    Large,   // 64x64 (Lv 51-98)
    Boss,    // 128x128 (Lv 99+)
}

impl Default for SpriteSize {
    fn default() -> Self {
        SpriteSize::Small
    }
}

impl From<&str> for SpriteSize {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "medium" => SpriteSize::Medium,
            "large" => SpriteSize::Large,
            "boss" => SpriteSize::Boss,
            _ => SpriteSize::Small,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "client", derive(bevy::prelude::Component))]
pub struct Monster {
    pub id: String,
    pub monster_id: i32,
    pub name: String,
    pub level: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    
    // 위치 및 상태
    pub position: Position,
    pub spawn_position: Position,
    pub direction: Direction,
    
    // AI
    pub ai_type: MonsterAIType,
    pub detection_range: f64,
    pub attack_range: f64,
    pub move_speed: f64,
    
    // 스프라이트 정보 (DB에서 로드)
    pub sprite_type: String,
    pub sprite_size: SpriteSize,
    
    // 전투 상태
    pub target_player_id: Option<String>,
    pub is_attacking: bool,
    pub last_attack_time: f64,
    pub attack_cooldown: f64,
}

impl Monster {
    pub fn new(monster_data: &MonsterData, position: Position) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            monster_id: monster_data.id,
            name: monster_data.name.clone(),
            level: monster_data.level,
            hp: monster_data.max_hp,
            max_hp: monster_data.max_hp,
            attack_min: monster_data.attack_min,
            attack_max: monster_data.attack_max,
            defense: monster_data.defense,
            exp_reward: monster_data.exp_reward,
            gold_min: monster_data.gold_min,
            gold_max: monster_data.gold_max,
            position,
            spawn_position: position,
            direction: Direction::Down,
            ai_type: monster_data.ai_type,
            detection_range: monster_data.detection_range,
            attack_range: monster_data.attack_range,
            move_speed: monster_data.move_speed,
            sprite_type: monster_data.sprite_type.clone(),
            sprite_size: monster_data.sprite_size,
            target_player_id: None,
            is_attacking: false,
            last_attack_time: 0.0,
            attack_cooldown: 1000.0,
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp - damage).max(0);
    }
    
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
    
    pub fn can_attack(&self, current_time: f64) -> bool {
        current_time - self.last_attack_time >= self.attack_cooldown
    }
    
    pub fn attack(&mut self, current_time: f64) -> i32 {
        self.last_attack_time = current_time;
        self.is_attacking = true;
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(self.attack_min..=self.attack_max)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    pub ai_type: MonsterAIType,
    pub detection_range: f64,
    pub attack_range: f64,
    pub move_speed: f64,
    pub sprite_path: String,
    pub sprite_type: String,
    pub sprite_size: SpriteSize,
}

/// DTO for receiving monster data from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct MonsterDataDto {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub hp_max: i32,
    pub mp_max: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    pub sprite_path: Option<String>,
    pub ai_type: Option<String>,
    pub sprite_type: Option<String>,
    pub sprite_size: Option<String>,
}

impl MonsterDataDto {
    /// Convert API DTO to game MonsterData
    /// NO HARDCODING - all data comes from database
    pub fn into_monster_data(self) -> MonsterData {
        let ai_type = match self.ai_type.as_deref() {
            Some("aggressive") => MonsterAIType::Aggressive,
            Some("defensive") => MonsterAIType::Defensive,
            _ => MonsterAIType::Passive,
        };
        
        // Default values based on AI type
        let (detection_range, attack_range, move_speed) = match ai_type {
            MonsterAIType::Aggressive => (250.0, 55.0, 110.0),
            MonsterAIType::Defensive => (200.0, 50.0, 100.0),
            MonsterAIType::Passive => (150.0, 40.0, 80.0),
        };
        
        // Sprite type from DB (no hardcoding!)
        let sprite_type = self.sprite_type.unwrap_or_else(|| "slime".to_string());
        
        // Sprite size from DB (no hardcoding!)
        let sprite_size = self.sprite_size
            .as_deref()
            .map(SpriteSize::from)
            .unwrap_or_default();
        
        // Sprite path from DB or generate from sprite_type
        let sprite_path = self.sprite_path.unwrap_or_else(|| {
            format!("/assets/monsters/{}/spritesheet.png", sprite_type)
        });
        
        MonsterData {
            id: self.id,
            name: self.name,
            level: self.level,
            max_hp: self.hp_max,
            attack_min: self.attack_min,
            attack_max: self.attack_max,
            defense: self.defense,
            exp_reward: self.exp_reward,
            gold_min: self.gold_min,
            gold_max: self.gold_max,
            ai_type,
            detection_range,
            attack_range,
            move_speed,
            sprite_path,
            sprite_type,
            sprite_size,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MonsterAIType {
    Passive,    // 공격하지 않음
    Aggressive, // 감지 범위 내 적극 공격
    Defensive,  // 공격받으면 반격
}
