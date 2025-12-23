//! Game Resources

use bevy::prelude::*;

/// Game configuration
#[derive(Resource)]
pub struct GameConfig {
    pub tile_size: f32,
    pub player_speed: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tile_size: 32.0,
            player_speed: 150.0,
        }
    }
}

/// Asset handles
#[derive(Resource, Default)]
pub struct GameAssets {
    pub ui_font: Handle<Font>,
    pub assets_loaded: bool,
}

/// Selected character class in character creation
#[derive(Resource, Default)]
pub struct SelectedClass {
    pub class: Option<crate::shared::domain::PlayerClass>,
    pub gender: String,
    pub username: String,
}

/// Skill definitions fetched from DB
#[derive(Resource, Default)]
pub struct SkillData {
    pub skills: Vec<crate::shared::domain::skill::models::Skill>,
}

/// Internationalization resource
#[derive(Resource)]
pub struct I18nResource {
    pub current_lang: String,
    pub pack: serde_json::Value,
}

impl Default for I18nResource {
    fn default() -> Self {
        Self {
            current_lang: "en".to_string(), // Default EN
            pack: serde_json::json!({}),
        }
    }
}

impl I18nResource {
    pub fn t(&self, key: &str) -> String {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &self.pack;
        
        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return key.to_string(); // Return key if not found
            }
        }
        
        current.as_str().unwrap_or(key).to_string()
    }
}
