//! Game Resources

use bevy::prelude::*;
use crate::shared::domain::{Player, PlayerClass};

/// Game configuration
#[derive(Resource, Default)]
pub struct GameConfig {
    pub tile_size: f32,
    pub player_speed: f32,
    pub api_base_url: String,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            tile_size: 32.0,
            player_speed: 150.0,
            api_base_url: "http://localhost:3000/api".to_string(),
        }
    }
}

/// Current player data
#[derive(Resource, Default)]
pub struct PlayerData {
    pub player: Option<Player>,
    pub is_logged_in: bool,
}

/// Asset handles
#[derive(Resource, Default)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub monster_sprites: Vec<Handle<Image>>,
    pub tileset: Handle<Image>,
    pub ui_font: Handle<Font>,
    pub assets_loaded: bool,
}

/// Selected character class in character creation
#[derive(Resource, Default)]
pub struct SelectedClass {
    pub class: Option<PlayerClass>,
    pub gender: String,
    pub username: String,
}
