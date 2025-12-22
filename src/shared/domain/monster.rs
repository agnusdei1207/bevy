use serde::{Deserialize, Serialize};
use super::Position;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub direction: super::Direction,
    
    // AI
    pub ai_type: MonsterAIType,
    pub detection_range: f64,
    pub attack_range: f64,
    pub move_speed: f64,
    
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
            direction: super::Direction::Down,
            ai_type: monster_data.ai_type,
            detection_range: monster_data.detection_range,
            attack_range: monster_data.attack_range,
            move_speed: monster_data.move_speed,
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
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MonsterAIType {
    Passive,    // 공격하지 않음
    Aggressive, // 감지 범위 내 적극 공격
    Defensive,  // 공격받으면 반격
}
