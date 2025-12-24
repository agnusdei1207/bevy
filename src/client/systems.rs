//! Core Game Systems

use bevy::prelude::*;
use bevy::asset::LoadState;
use super::resources::*;
use super::states::GameState;
use std::collections::HashMap;

/// Loading state tracker
#[derive(Resource)]
pub struct LoadingState {
    pub start_time: f64,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            start_time: 0.0,
        }
    }
}

/// Setup 2D camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // Ensure camera is positioned correctly to see Z-ordered sprites
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

/// Load game assets
pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    info!("📦 Starting asset loading...");

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
            // Prefer folder structure: characters/warrior/male/spritesheet.png
            // Fallback to characters/warrior/male_spritesheet.png if needed, but let's try strict first
             let path = format!("characters/{}/{}/spritesheet.png", class_name, gender);
            // let path = format!("characters/{}/{}_spritesheet.png", class_name, gender);
            let handle = asset_server.load(&path);
            gender_map.insert(gender.to_string(), handle);
        }
        game_assets.character_sprites.insert(class_name.to_string(), gender_map);
    }
}

/// Check if assets are loaded and transition to main menu
pub fn check_assets_loaded(
    mut game_assets: ResMut<GameAssets>,
    skill_data: Res<SkillData>,
    monster_definitions: Res<MonsterDefinitions>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut loading_state: ResMut<LoadingState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 1. Initialize start time
    if loading_state.start_time == 0.0 {
        loading_state.start_time = time.elapsed_secs_f64();
        info!("⏳ Waiting for assets to load...");
    }

    let elapsed = time.elapsed_secs_f64() - loading_state.start_time;

    // 2. Check essential assets (Font)
    let font_state = asset_server.get_load_state(&game_assets.ui_font);
    
    // 3. Track progress
    let mut pending_count = 0;
    let mut _failed_count = 0;
    let mut font_ready = false;
    
    // Check font
    match font_state {
        Some(LoadState::Loaded) => {
            font_ready = true;
        },
        Some(LoadState::Failed(ref e)) => { // Use ref to avoid move
            warn!("❌ Font load failed: {:?}", e);
            _failed_count += 1; 
        },
        _ => { pending_count += 1; }
    }

    // Check optional assets (just for logging, don't block too long)
    if let Some(atlas) = &game_assets.tile_atlas {
        if !matches!(asset_server.get_load_state(atlas), Some(LoadState::Loaded)) {
            // checking...
        }
    }

    // 4. Decision logic
    let data_ready = !skill_data.skills.is_empty() && !monster_definitions.definitions.is_empty();
    
    // Proceed if:
    // A) Everything is ready
    // B) Timeout passed (5 seconds) -> Force start
    
    if font_ready && data_ready && pending_count == 0 {
        info!("✅ All assets loaded successfully! ({}s)", elapsed);
        game_assets.assets_loaded = true;
        next_state.set(GameState::MainMenu);
    } else if elapsed > 5.0 {
        warn!("⏰ Timeout ({}s). Force starting game even if assets are missing.", elapsed);
        warn!("   - Font loaded: {} (State: {:?})", font_ready, font_state);
        warn!("   - Data ready: {}", data_ready);
        
        game_assets.assets_loaded = true; // Pretend we are loaded
        next_state.set(GameState::MainMenu);
    } else {
        // Still loading... show progress every second
        let secs = elapsed as u64;
        if elapsed - (secs as f64) < 0.05 && secs > 0 {
           debug!("... loading ({}s elapsed)", secs);
        }
    }
}

