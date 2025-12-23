//! Game Components (ECS)

use bevy::prelude::*;
use crate::shared::domain::{Direction, PlayerClass, MonsterAIType};

// ============ Player Components ============

/// Marker component for the player entity
#[derive(Component)]
pub struct PlayerComponent;

// ============ Monster Components ============

/// Marker component for monster entities
#[derive(Component)]
pub struct MonsterComponent;

/// Monster AI behavior
#[derive(Component)]
pub struct MonsterAI {
    pub ai_type: MonsterAIType,
    pub detection_range: f32,
    pub attack_range: f32,
    #[allow(dead_code)]
    pub move_speed: f32,
    #[allow(dead_code)]
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
#[allow(dead_code)]
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
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_frame: 0,
            frame_count: 4,
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
        }
    }
}

/// Combat state component
#[derive(Component)]
pub struct CombatState {
    #[allow(dead_code)]
    pub is_attacking: bool,
    #[allow(dead_code)]
    pub attack_timer: Timer,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            is_attacking: false,
            attack_timer: Timer::from_seconds(1.0, TimerMode::Once),
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
    CharacterSelect,
    SelectClass(PlayerClass),
    ConfirmCharacter,
    BackToMenu,
    Quit,
    ChangeLanguage(String),
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

/// Logical grid position
#[derive(Component, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

/// Target logical grid position for movement
#[derive(Component, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct TargetGridPosition {
    pub x: i32,
    pub y: i32,
}

/// Movement progression between grid tiles (0.0 to 1.0)
#[derive(Component, Default)]
pub struct MovementProgress {
    pub timer: Timer,
    pub start_pos: Vec2, // Logical grid coords as f32
}

/// Tile component
#[derive(Component)]
pub struct TileComponent {
    pub tile_type: TileType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Stone,
    Fountain,
    Wall,
    Door,
}

/// Map component
#[derive(Component)]
#[allow(dead_code)]
pub struct TileMap {
    pub width: i32,
    pub height: i32,
}

/// Camera follow target
#[derive(Component)]
pub struct CameraTarget;

// ============ Interaction Components ============

/// Something the player can interact with
#[derive(Component)]
pub struct Interactable {
    pub message: String,
    pub interaction_type: InteractionType,
}

#[derive(Debug, Clone)]
pub enum InteractionType {
    NpcChat(String),
    #[allow(dead_code)]
    Portal { target_map: String, target_pos: GridPosition },
    #[allow(dead_code)]
    PickUp,
}

/// Marker for portal tiles (Doors)
#[derive(Component)]
#[allow(dead_code)]
pub struct Portal {
    pub target_map: String,
    pub target_pos: GridPosition,
}

// ============ Skill Components ============

#[derive(Component, Default)]
pub struct ActiveSkills {
    pub skills: Vec<SkillCooldown>,
}

pub struct SkillCooldown {
    pub skill_id: i32,
    pub timer: Timer,
}
