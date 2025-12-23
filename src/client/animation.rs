//! Animation System
//!
//! Sprite-based animation system for characters and monsters.

use bevy::prelude::*;
use std::collections::HashMap;

/// Animation configuration for sprite sheets
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// Number of columns in sprite sheet
    pub columns: usize,
    /// Number of rows in sprite sheet  
    pub rows: usize,
    /// Frame width in pixels
    pub frame_width: u32,
    /// Frame height in pixels
    pub frame_height: u32,
    /// Frames per second
    pub fps: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            columns: 4,
            rows: 4,
            frame_width: 48,
            frame_height: 64,
            fps: 8.0,
        }
    }
}

/// Character animation configurations per class
pub struct CharacterAnimations;

impl CharacterAnimations {
    pub fn get_config(_class: &str) -> AnimationConfig {
        // All character classes use same layout for consistency
        AnimationConfig {
            columns: 4,
            rows: 4,
            frame_width: 48,
            frame_height: 64,
            fps: 8.0,
        }
    }
}

/// Monster animation configurations based on sprite size
pub struct MonsterAnimations;

impl MonsterAnimations {
    pub fn get_config(sprite_size: &crate::shared::domain::monster::SpriteSize) -> AnimationConfig {
        use crate::shared::domain::monster::SpriteSize;
        match sprite_size {
            SpriteSize::Small => AnimationConfig {
                columns: 4,
                rows: 4,
                frame_width: 32,
                frame_height: 32,
                fps: 8.0,
            },
            SpriteSize::Medium => AnimationConfig {
                columns: 4,
                rows: 4,
                frame_width: 48,
                frame_height: 48,
                fps: 8.0,
            },
            SpriteSize::Large => AnimationConfig {
                columns: 4,
                rows: 4,
                frame_width: 64,
                frame_height: 64,
                fps: 6.0,
            },
            SpriteSize::Boss => AnimationConfig {
                columns: 4,
                rows: 4,
                frame_width: 128,
                frame_height: 128,
                fps: 6.0,
            },
        }
    }
}

/// Animation state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnimState {
    #[default]
    Idle,
    Walk,
    Attack,
    Hit,
    Die,
}

impl AnimState {
    /// Get the row index for this animation state in a standard spritesheet
    pub fn row_index(&self) -> usize {
        match self {
            AnimState::Idle => 0,
            AnimState::Walk => 1,
            AnimState::Attack => 2,
            AnimState::Hit => 3,
            AnimState::Die => 3, // Reuse hit row for death
        }
    }
}

/// Direction for 4-directional sprites
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnimDirection {
    #[default]
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3,
}

impl From<crate::shared::domain::Direction> for AnimDirection {
    fn from(dir: crate::shared::domain::Direction) -> Self {
        use crate::shared::domain::Direction;
        match dir {
            Direction::Down => AnimDirection::Down,
            Direction::Left => AnimDirection::Left,
            Direction::Right => AnimDirection::Right,
            Direction::Up => AnimDirection::Up,
        }
    }
}

/// Animator component for entities with sprite animations
#[derive(Component)]
pub struct Animator {
    /// Current animation state
    pub state: AnimState,
    /// Current direction
    pub direction: AnimDirection,
    /// Current frame in the animation
    pub current_frame: usize,
    /// Animation timer
    pub timer: Timer,
    /// Number of frames in current animation
    pub frame_count: usize,
    /// Is animation playing?
    pub playing: bool,
    /// Loop animation?
    pub looping: bool,
    /// Configuration
    pub config: AnimationConfig,
}

impl Default for Animator {
    fn default() -> Self {
        Self {
            state: AnimState::Idle,
            direction: AnimDirection::Down,
            current_frame: 0,
            timer: Timer::from_seconds(1.0 / 8.0, TimerMode::Repeating),
            frame_count: 4,
            playing: true,
            looping: true,
            config: AnimationConfig::default(),
        }
    }
}

impl Animator {
    pub fn new(config: AnimationConfig) -> Self {
        let fps = config.fps;
        Self {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            frame_count: config.columns,
            config,
            ..default()
        }
    }

    /// Set animation state (e.g., Idle -> Walk)
    pub fn set_state(&mut self, state: AnimState) {
        if self.state != state {
            self.state = state;
            self.current_frame = 0;
            self.timer.reset();
            self.playing = true;
            
            // Attack animation doesn't loop
            self.looping = !matches!(state, AnimState::Attack | AnimState::Hit | AnimState::Die);
        }
    }

    /// Set facing direction
    pub fn set_direction(&mut self, direction: AnimDirection) {
        self.direction = direction;
    }

    /// Get the current sprite index for the texture atlas
    pub fn sprite_index(&self) -> usize {
        let row = self.state.row_index();
        let col = self.current_frame.min(self.frame_count.saturating_sub(1));
        row * self.config.columns + col
    }

    /// Advance animation by one frame
    pub fn advance(&mut self) {
        if !self.playing {
            return;
        }

        self.current_frame += 1;
        if self.current_frame >= self.frame_count {
            if self.looping {
                self.current_frame = 0;
            } else {
                self.current_frame = self.frame_count - 1;
                self.playing = false;
            }
        }
    }

    /// Check if non-looping animation has finished
    pub fn is_finished(&self) -> bool {
        !self.looping && !self.playing
    }
}

/// System to update animations
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&mut Animator, &mut Sprite)>,
) {
    for (mut animator, mut sprite) in &mut query {
        if !animator.playing {
            continue;
        }

        animator.timer.tick(time.delta());
        if animator.timer.just_finished() {
            animator.advance();
            
            // Update sprite rect based on current frame
            let frame_w = animator.config.frame_width as f32;
            let frame_h = animator.config.frame_height as f32;
            let col = animator.current_frame % animator.config.columns;
            let row = animator.state.row_index();
            
            sprite.rect = Some(Rect::new(
                col as f32 * frame_w,
                row as f32 * frame_h,
                (col + 1) as f32 * frame_w,
                (row + 1) as f32 * frame_h,
            ));
        }
    }
}

/// Texture atlas cache resource
#[derive(Resource, Default)]
pub struct AtlasCache {
    #[allow(dead_code)]
    pub monster_layouts: HashMap<String, Handle<TextureAtlasLayout>>,
    #[allow(dead_code)]
    pub character_layouts: HashMap<String, Handle<TextureAtlasLayout>>,
}

/// Create a texture atlas layout for a sprite sheet
pub fn create_atlas_layout(
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    config: &AnimationConfig,
) -> Handle<TextureAtlasLayout> {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.frame_width, config.frame_height),
        config.columns as u32,
        config.rows as u32,
        None,
        None,
    );
    texture_atlas_layouts.add(layout)
}
