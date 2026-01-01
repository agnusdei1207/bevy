//! Animation System
//!
//! Sprite-based animation system for characters and monsters.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::shared::constants::*;

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
            columns: SPRITESHEET_COLUMNS,
            rows: SPRITESHEET_ROWS,
            frame_width: CHARACTER_FRAME_SIZE,
            frame_height: CHARACTER_FRAME_SIZE,
            fps: ANIMATION_FPS_DEFAULT,
        }
    }
}

/// Character animation configurations per class
/// Standard: 256x256 sheet, 64x64 frames, 4x4 grid
pub struct CharacterAnimations;

impl CharacterAnimations {
    pub fn get_config(_class: &str) -> AnimationConfig {
        AnimationConfig {
            columns: SPRITESHEET_COLUMNS,
            rows: SPRITESHEET_ROWS,
            frame_width: CHARACTER_FRAME_SIZE,
            frame_height: CHARACTER_FRAME_SIZE,
            fps: ANIMATION_FPS_DEFAULT,
        }
    }
}

/// Monster animation configurations based on sprite size
pub struct MonsterAnimations;

impl MonsterAnimations {
    pub fn get_config(sprite_size: &crate::shared::domain::monster::SpriteSize) -> AnimationConfig {
        use crate::shared::domain::monster::SpriteSize;
        let frame_size = match sprite_size {
            SpriteSize::Small => 32,
            SpriteSize::Medium => 48,
            SpriteSize::Large => 64,
            SpriteSize::Boss => 128,
        };
        
        AnimationConfig {
            columns: SPRITESHEET_COLUMNS,
            rows: SPRITESHEET_ROWS,
            frame_width: frame_size,
            frame_height: frame_size,
            fps: ANIMATION_FPS_DEFAULT,
        }
    }
}

/// Animator component for entities with sprite animations
#[derive(Debug, Clone, Component)]
pub struct SpriteAnimator {
    pub state: crate::shared::domain::sprite::AnimationState,
    pub direction: crate::shared::domain::sprite::SpriteDirection,
    pub current_frame: usize,
    pub timer: Timer,
    pub playing: bool,
    pub looping: bool,
    pub manifest: Option<Handle<crate::shared::domain::sprite::SpriteManifest>>,
}

impl Default for SpriteAnimator {
    fn default() -> Self {
        Self {
            state: crate::shared::domain::sprite::AnimationState::Idle,
            direction: crate::shared::domain::sprite::SpriteDirection::Down,
            current_frame: 0,
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            playing: true,
            looping: true,
            manifest: None,
        }
    }
}

impl SpriteAnimator {
    /// Initialize sprite with the first frame using manifest
    pub fn initialize_sprite(&self, sprite: &mut Sprite, manifests: &Assets<crate::shared::domain::sprite::SpriteManifest>) {
        let Some(handle) = &self.manifest else { return; };
        let Some(manifest) = manifests.get(handle) else { return; };
        
        // Get first frame (index 0) of current state and direction
        let (index, flip) = manifest.get_frame_index(self.state, self.direction, 0);
        
        // Set flip
        sprite.flip_x = flip;
        
        // Calculate and set rect
        let (rect_x, rect_y, w, h) = manifest.layout.get_frame_rect(index);
        sprite.rect = Some(Rect::new(
            rect_x as f32,
            rect_y as f32,
            (rect_x + w) as f32,
            (rect_y + h) as f32,
        ));
        
        // Set custom size to match frame size
        sprite.custom_size = Some(Vec2::new(w as f32, h as f32));
    }
}

/// System to update animations using SpriteManifest
pub fn update_animations(
    time: Res<Time>,
    manifests: Res<Assets<crate::shared::domain::sprite::SpriteManifest>>,
    mut query: Query<(&mut SpriteAnimator, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in &mut query {
        if !anim.playing {
            continue;
        }

        anim.timer.tick(time.delta());
        
        let Some(handle) = &anim.manifest else { continue; };
        let Some(manifest) = manifests.get(handle) else { continue; };
        
        // Get animation sequence
        let Some(seq) = manifest.get_animation(anim.state) else { continue; };
        
        // Sync timer duration if it doesn't match the manifest FPS
        let target_duration = 1.0 / seq.fps;
        if (anim.timer.duration().as_secs_f32() - target_duration).abs() > 0.001 {
            anim.timer.set_duration(std::time::Duration::from_secs_f32(target_duration));
        }

        if anim.timer.just_finished() {
            anim.current_frame += 1;
            
            if anim.current_frame >= seq.frame_count {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame = seq.frame_count - 1;
                    anim.playing = false;
                }
            }
        }

        // Calculate frame index and flip
        let (index, flip) = manifest.get_frame_index(anim.state, anim.direction, anim.current_frame);
        
        // Update sprite
        sprite.flip_x = flip;
        
        // Calculate Rect from index
        let (rect_x, rect_y, w, h) = manifest.layout.get_frame_rect(index);
        sprite.rect = Some(Rect::new(
            rect_x as f32,
            rect_y as f32,
            (rect_x + w) as f32,
            (rect_y + h) as f32,
        ));
        
        // Ensure custom size matches frame size to avoid box squashing
        sprite.custom_size = Some(Vec2::new(w as f32, h as f32));
    }
}

/// System to initialize newly spawned sprites (runs once on spawn)
/// This prevents the "flashing all frames" issue by setting initial rect
pub fn initialize_new_sprites(
    manifests: Res<Assets<crate::shared::domain::sprite::SpriteManifest>>,
    mut query: Query<(&SpriteAnimator, &mut Sprite), Added<SpriteAnimator>>,
) {
    for (animator, mut sprite) in &mut query {
        animator.initialize_sprite(&mut sprite, &manifests);
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
