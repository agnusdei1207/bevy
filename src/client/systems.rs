//! Core Game Systems

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::states::GameState;

/// Setup 2D camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

/// Load game assets
pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    // Load default font
    game_assets.ui_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // For now, we'll use colored sprites instead of actual images
    // This can be replaced with actual sprite loading later
    game_assets.assets_loaded = true;
}

/// Check if assets are loaded and transition to main menu
pub fn check_assets_loaded(
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_assets.assets_loaded {
        next_state.set(GameState::MainMenu);
    }
}
