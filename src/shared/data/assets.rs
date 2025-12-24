//! Asset path constants
//!
//! Centralized asset path management for sprites, audio, and other resources.

/// Base paths for different asset categories
pub mod paths {
    /// Base path for all assets (relative to public directory)
    pub const ASSETS_BASE: &str = "/assets";
    
    /// Tile assets
    pub const TILES_BASE: &str = "/assets/tiles";
    pub const TILES_GROUND: &str = "/assets/tiles/ground";
    pub const TILES_BUILDINGS: &str = "/assets/tiles/buildings";
    pub const TILES_DECORATIONS: &str = "/assets/tiles/decorations";
    
    /// Character assets
    pub const CHARACTERS_BASE: &str = "/assets/characters";
    
    /// Monster assets
    pub const MONSTERS_BASE: &str = "/assets/monsters";
    
    /// UI assets
    pub const UI_BASE: &str = "/assets/ui";
    
    /// Audio assets
    pub const AUDIO_BASE: &str = "/assets/audio";
    
    /// Font assets
    pub const FONTS_BASE: &str = "/assets/fonts";
    
    /// Skill assets
    pub const SKILLS_BASE: &str = "/assets/skills";
}

/// Ground tileset sprite sheet configuration
pub struct TilesetConfig {
    pub path: &'static str,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

/// Tile indices in the tileset spritesheet
pub mod tile_indices {
    // Row 0: Basic ground tiles
    pub const GRASS: u32 = 0;
    pub const GRASS_DARK: u32 = 1;
    pub const STONE: u32 = 2;
    pub const STONE_DARK: u32 = 3;
    pub const COBBLESTONE: u32 = 4;
    pub const DIRT: u32 = 5;
    
    // Row 1: Special tiles
    pub const WATER: u32 = 8;
    pub const WATER_EDGE: u32 = 9;
    pub const LAVA: u32 = 10;
    pub const ICE: u32 = 11;
    
    // Row 2: Wall tiles
    pub const WALL_STONE: u32 = 16;
    pub const WALL_BRICK: u32 = 17;
    pub const WALL_WOOD: u32 = 18;
    pub const DOOR_CLOSED: u32 = 19;
    pub const DOOR_OPEN: u32 = 20;
}

pub const GROUND_TILESET: TilesetConfig = TilesetConfig {
    path: "/assets/tiles/ground/tileset.png",
    tile_width: 64,
    tile_height: 32, // Isometric 2:1 ratio
    columns: 8,
    rows: 8,
};

/// Building sprite configurations  
pub struct BuildingConfig {
    pub path: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const BUILDINGS_SPRITESHEET: &str = "/assets/tiles/buildings/buildings.png";

pub mod building_indices {
    pub const HOUSE_SMALL: u32 = 0;
    pub const HOUSE_MEDIUM: u32 = 1;
    pub const BLACKSMITH: u32 = 2;
    pub const TAVERN: u32 = 3;
    pub const GUILD_HALL: u32 = 4;
    pub const SHOP: u32 = 5;
}

/// Decoration sprite paths
pub mod decorations {
    pub const TORCH: &str = "/assets/tiles/decorations/torch.png";
    pub const FOUNTAIN: &str = "/assets/tiles/decorations/fountain.png";
    pub const TREE: &str = "/assets/tiles/decorations/tree.png";
    pub const BUSH: &str = "/assets/tiles/decorations/bush.png";
    pub const ROCK: &str = "/assets/tiles/decorations/rock.png";
    pub const GRAVE: &str = "/assets/tiles/decorations/grave.png";
    pub const MAGIC_CIRCLE: &str = "/assets/tiles/decorations/magic_circle.png";
}

/// Font paths
pub mod fonts {
    pub const DEFAULT: &str = "/assets/fonts/NotoSansKR-Regular.ttf";
    pub const TITLE: &str = "/assets/fonts/NotoSansKR-Bold.ttf";
}

/// Audio paths
pub mod audio {
    pub const BGM_VILLAGE: &str = "/assets/audio/bgm/village.mp3";
    pub const BGM_DUNGEON: &str = "/assets/audio/bgm/dungeon.mp3";
    pub const BGM_BATTLE: &str = "/assets/audio/bgm/battle.mp3";
    pub const SFX_ATTACK: &str = "/assets/audio/sfx_attack.wav";
    pub const SFX_HIT: &str = "/assets/audio/sfx_hit.wav";
    pub const SFX_DEATH: &str = "/assets/audio/sfx_death.wav";
    pub const SFX_LEVEL_UP: &str = "/assets/audio/sfx_levelup.wav";
}
