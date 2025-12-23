//! Core Game Systems

use bevy::prelude::*;
use super::resources::*;
use super::states::GameState;
use std::collections::HashMap;

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
    game_assets.ui_font = asset_server.load("fonts/NanumGothic.ttf");
    
    // Load tile atlas
    game_assets.tile_atlas = Some(asset_server.load("tiles/ground/tileset.png"));
    game_assets.buildings_atlas = Some(asset_server.load("tiles/buildings/buildings.png"));
    
    // Load decoration sprites
    game_assets.torch_sprite = Some(asset_server.load("tiles/decorations/torch.png"));
    
    // Load monster sprites
    let monster_types = ["rat", "bat", "slime", "wolf", "skeleton", "goblin", "ghost", "dragon"];
    for monster_type in monster_types {
        let path = format!("monsters/{}/spritesheet.png", monster_type);
        let handle = asset_server.load(&path);
        game_assets.monster_sprites.insert(monster_type.to_string(), handle);
    }
    
    // Load character sprites
    let classes = ["warrior", "rogue", "mage", "cleric", "martial_artist"];
    let genders = ["male", "female"];
    
    for class_name in classes {
        let mut gender_map = HashMap::new();
        for gender in genders {
            let path = format!("characters/{}/{}_spritesheet.png", class_name, gender);
            let handle = asset_server.load(&path);
            gender_map.insert(gender.to_string(), handle);
        }
        game_assets.character_sprites.insert(class_name.to_string(), gender_map);
    }
    
    println!("📦 Loading assets...");
    println!("  - {} monster sprites", game_assets.monster_sprites.len());
    println!("  - {} character classes", game_assets.character_sprites.len());
    
    game_assets.assets_loaded = true;
}

/// Check if assets are loaded and transition to main menu
pub fn check_assets_loaded(
    game_assets: Res<GameAssets>,
    skill_data: Res<SkillData>,
    monster_definitions: Res<MonsterDefinitions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Log once
    static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !LOGGED.load(std::sync::atomic::Ordering::Relaxed) {
        println!("✅ Skills loaded: {}", skill_data.skills.len());
        println!("✅ Monsters loaded: {}", monster_definitions.definitions.len());
        LOGGED.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    // Check if ready
    if game_assets.assets_loaded && !skill_data.skills.is_empty() && !monster_definitions.definitions.is_empty() {
        println!("🎮 All assets loaded! Starting game...");
        next_state.set(GameState::MainMenu);
    }
}
