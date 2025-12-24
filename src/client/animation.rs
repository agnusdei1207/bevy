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
