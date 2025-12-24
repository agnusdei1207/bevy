//! Legend of Darkness M - Bevy Game Client
//!
//! This module contains the Bevy game implementation.

mod game;
mod ui;
mod systems;
mod states;
mod components;
mod resources;
pub mod animation;
pub mod inventory;

use bevy::prelude::*;
use states::GameState;

/// Main game plugin that sets up all game systems
pub struct LegendGamePlugin;

impl Plugin for LegendGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Game states
            .init_state::<GameState>()
            
            // Resources (Global)
            .insert_resource(resources::GameConfig::default())
            .insert_resource(resources::GameAssets::default())
            .insert_resource(resources::SelectedClass::default())
            .insert_resource(resources::SkillData::default())
            .insert_resource(resources::TextResource::default())
            .insert_resource(resources::MonsterDefinitions::default())
            .insert_resource(resources::SpriteAtlases::default())
            .insert_resource(systems::LoadingState::default())
            
            // Startup systems
            .add_systems(Startup, (
                systems::setup_camera,
            ))
            
            // Loading state
            .add_systems(OnEnter(GameState::Loading), systems::load_assets)
            .add_systems(Update, (
                systems::check_assets_loaded,
            ).run_if(in_state(GameState::Loading)))
            
            // Main menu state
            .add_systems(OnEnter(GameState::MainMenu), ui::spawn_main_menu)
            .add_systems(Update, ui::main_menu_interaction.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), ui::cleanup_main_menu)
            
            // Character select state
            .add_systems(OnEnter(GameState::CharacterSelect), ui::spawn_character_select)
            .add_systems(Update, ui::character_select_interaction.run_if(in_state(GameState::CharacterSelect)))
            .add_systems(OnExit(GameState::CharacterSelect), ui::cleanup_character_select)
            
            // Playing state
            .add_systems(OnEnter(GameState::Playing), game::spawn_game_world)
            .add_systems(Update, (
                game::player_movement,
                game::character_grid_movement,
                game::player_animation,
                game::camera_follow,
                game::monster_ai,
                game::skill_system,
                game::interaction_system,
                animation::update_animations,
                ui::update_hud,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), game::cleanup_game_world);
    }
}
