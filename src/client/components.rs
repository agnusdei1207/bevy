//! Game Components (ECS)

use bevy::prelude::*;
use crate::shared::domain::{Direction, PlayerClass, MonsterAIType};

// ============ Player Components ============

/// Marker component for the player entity
#[derive(Component)]
pub struct PlayerComponent;

/// Player stats component
#[derive(Component)]
pub struct PlayerStats {
    pub class: PlayerClass,
    pub level: i32,
    pub exp: i64,
    pub exp_to_next: i64,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub gold: i64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            class: PlayerClass::Warrior,
            level: 1,
            exp: 0,
            exp_to_next: 100,
            hp: 120,
            max_hp: 120,
            mp: 60,
            max_mp: 60,
            attack: 15,
            defense: 10,
            gold: 100,
        }
    }
}

// ============ Monster Components ============

/// Marker component for monster entities
#[derive(Component)]
pub struct MonsterComponent {
    pub monster_id: i32,
    pub name: String,
}

/// Monster stats
#[derive(Component)]
pub struct MonsterStats {
    pub level: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
}

/// Monster AI behavior
#[derive(Component)]
pub struct MonsterAI {
    pub ai_type: MonsterAIType,
    pub detection_range: f32,
    pub attack_range: f32,
    pub move_speed: f32,
    pub target: Option<Entity>,
    pub spawn_position: Vec2,
}

// ============ Common Components ============

/// Movement/Direction component
#[derive(Component)]
pub struct Facing {
    pub direction: Direction,
}

impl Default for Facing {
    fn default() -> Self {
        Self {
            direction: Direction::Down,
        }
    }
}

/// Velocity component for movement
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Animation component
#[derive(Component)]
pub struct AnimationState {
    pub current_frame: usize,
    pub frame_count: usize,
    pub timer: Timer,
    pub is_playing: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_frame: 0,
            frame_count: 4,
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            is_playing: true,
        }
    }
}

/// Combat state component
#[derive(Component)]
pub struct CombatState {
    pub is_attacking: bool,
    pub attack_timer: Timer,
    pub target: Option<Entity>,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            is_attacking: false,
            attack_timer: Timer::from_seconds(1.0, TimerMode::Once),
            target: None,
        }
    }
}

// ============ UI Components ============

/// Marker for main menu UI
#[derive(Component)]
pub struct MainMenuUI;

/// Marker for character select UI
#[derive(Component)]
pub struct CharacterSelectUI;

/// Marker for HUD elements
#[derive(Component)]
pub struct HudUI;

/// Button action types
#[derive(Component, Clone)]
pub enum ButtonAction {
    StartGame,
    CharacterSelect,
    SelectClass(PlayerClass),
    ConfirmCharacter,
    BackToMenu,
    Quit,
}

/// HP Bar UI
#[derive(Component)]
pub struct HpBar;

/// MP Bar UI
#[derive(Component)]
pub struct MpBar;

/// Exp Bar UI  
#[derive(Component)]
pub struct ExpBar;

/// Level text
#[derive(Component)]
pub struct LevelText;

/// Gold text
#[derive(Component)]
pub struct GoldText;

// ============ World Components ============

/// Tile component
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Grass,
    Stone,
    Water,
    Wall,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        matches!(self, TileType::Grass | TileType::Stone)
    }
}

/// Camera follow target
#[derive(Component)]
pub struct CameraTarget;
