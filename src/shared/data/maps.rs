//! Map data constants
//!
//! All map definitions including tile layouts, spawn points, and NPCs.

/// Map definition
#[derive(Debug, Clone)]
pub struct MapDef {
    pub id: &'static str,
    pub name: &'static str,
    pub name_key: &'static str,
    pub description_key: &'static str,
    pub width: usize,
    pub height: usize,
    pub tile_size: usize,
    pub min_level: i32,
    pub max_level: i32,
    pub pvp_enabled: bool,
    pub is_dungeon: bool,
    pub bgm_path: &'static str,
}

/// Tile layout for maps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTile {
    Grass,      // 'G' - Walkable grass
    Stone,      // 'S' - Walkable stone path
    Water,      // 'W' - Non-walkable water
    Wall,       // 'X' - Non-walkable wall
    Door,       // 'D' - Door/portal
    Fountain,   // 'F' - Fountain (decoration)
    Tree,       // 'T' - Tree (non-walkable)
    Building,   // 'B' - Building (non-walkable)
}

impl MapTile {
    pub fn from_char(c: char) -> Self {
        match c {
            'G' => MapTile::Grass,
            'S' => MapTile::Stone,
            'W' => MapTile::Water,
            'X' => MapTile::Wall,
            'D' => MapTile::Door,
            'F' => MapTile::Fountain,
            'T' => MapTile::Tree,
            'B' => MapTile::Building,
            _ => MapTile::Grass,
        }
    }
    
    pub fn is_walkable(&self) -> bool {
        matches!(self, MapTile::Grass | MapTile::Stone | MapTile::Door)
    }
}

/// Monster spawn point
#[derive(Debug, Clone)]
pub struct SpawnPoint {
    pub x: i32,
    pub y: i32,
    pub monster_name: &'static str,
    pub respawn_time_ms: u64,
}

/// NPC definition for maps
#[derive(Debug, Clone)]
pub struct NpcDef {
    pub id: &'static str,
    pub name_key: &'static str,
    pub x: i32,
    pub y: i32,
    pub interaction_type: NpcType,
    pub dialogue_key: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    Shop,
    Inn,
    Blacksmith,
    QuestGiver,
    Guide,
}

/// Portal definition
#[derive(Debug, Clone)]
pub struct PortalDef {
    pub x: i32,
    pub y: i32,
    pub target_map: &'static str,
    pub target_x: i32,
    pub target_y: i32,
}

// ============ Village Milles (Starting Town) ============

pub const VILLAGE_MILLES: MapDef = MapDef {
    id: "village_milles",
    name: "Milles Village",
    name_key: "map.village_milles",
    description_key: "map.village_milles.desc",
    width: 16,
    height: 16,
    tile_size: 64,
    min_level: 1,
    max_level: 99,
    pvp_enabled: false,
    is_dungeon: false,
    bgm_path: "/assets/audio/bgm/village.mp3",
};

/// Milles Village tile layout (16x16)
pub const MILLES_LAYOUT: [&str; 16] = [
    "GGGGGGSSSSGGGGGG", // 0 - North area
    "GGBWGSSSSGGBWWGG", // 1 - Buildings with walls
    "GGDWGSSSSGWDDWGG", // 2 - Doors
    "GGGGGGSSSSGGGGGG", // 3
    "SSSSSSSSSSSSSSSS", // 4 - Main street
    "SSSSSFFFSSSSSSSS", // 5 - Fountain area
    "SSSSSFFFSSSSSSSS", // 6
    "SSSSSFFFSSSSSSSS", // 7
    "SSSSSSSSSSSSSSSS", // 8 - Main street
    "GGGGGGSSSSGGGGGG", // 9
    "GBWGGSSSSGGBWWWG", // 10 - Buildings
    "GBDDGSSSSGWWDDWG", // 11 - Doors
    "GGGGGGSSSSGGGGGG", // 12
    "GGGGGGSSSSGGGGGG", // 13
    "TTTTGGSSSSGGTTTT", // 14 - Trees
    "TTTTGGSSSSGGTTTT", // 15 - Trees
];

/// Milles NPCs
pub const MILLES_NPCS: &[NpcDef] = &[
    NpcDef {
        id: "innkeeper_milles",
        name_key: "npc.innkeeper",
        x: 2,
        y: 2,
        interaction_type: NpcType::Inn,
        dialogue_key: "npc.innkeeper.dialogue",
    },
    NpcDef {
        id: "shopkeeper_milles",
        name_key: "npc.shopkeeper",
        x: 12,
        y: 11,
        interaction_type: NpcType::Shop,
        dialogue_key: "npc.shopkeeper.dialogue",
    },
    NpcDef {
        id: "blacksmith_milles",
        name_key: "npc.blacksmith",
        x: 13,
        y: 2,
        interaction_type: NpcType::Blacksmith,
        dialogue_key: "npc.blacksmith.dialogue",
    },
];

/// Portals from Milles
pub const MILLES_PORTALS: &[PortalDef] = &[
    PortalDef {
        x: 7,
        y: 15,
        target_map: "hunting_ground_1",
        target_x: 8,
        target_y: 1,
    },
];

// ============ Beginner Hunting Ground ============

pub const HUNTING_GROUND_1: MapDef = MapDef {
    id: "hunting_ground_1",
    name: "Beginner Hunting Ground",
    name_key: "map.hunting_ground_1",
    description_key: "map.hunting_ground_1.desc",
    width: 20,
    height: 20,
    tile_size: 64,
    min_level: 1,
    max_level: 10,
    pvp_enabled: false,
    is_dungeon: false,
    bgm_path: "/assets/audio/bgm/field.mp3",
};

/// Hunting ground spawns
pub const HUNTING_GROUND_1_SPAWNS: &[SpawnPoint] = &[
    SpawnPoint { x: 3, y: 3, monster_name: "Giant Rat", respawn_time_ms: 30000 },
    SpawnPoint { x: 3, y: 13, monster_name: "Vampire Bat", respawn_time_ms: 30000 },
    SpawnPoint { x: 13, y: 3, monster_name: "Slime", respawn_time_ms: 45000 },
    SpawnPoint { x: 13, y: 13, monster_name: "Slime", respawn_time_ms: 45000 },
    SpawnPoint { x: 8, y: 8, monster_name: "Giant Rat", respawn_time_ms: 30000 },
    SpawnPoint { x: 16, y: 8, monster_name: "Vampire Bat", respawn_time_ms: 30000 },
];

// ============ Dungeon 1: Slime Cave ============

pub const SLIME_CAVE: MapDef = MapDef {
    id: "slime_cave",
    name: "Slime Cave",
    name_key: "map.slime_cave",
    description_key: "map.slime_cave.desc",
    width: 24,
    height: 24,
    tile_size: 64,
    min_level: 10,
    max_level: 20,
    pvp_enabled: false,
    is_dungeon: true,
    bgm_path: "/assets/audio/bgm/dungeon.mp3",
};

/// All map definitions
pub const ALL_MAPS: &[&MapDef] = &[
    &VILLAGE_MILLES,
    &HUNTING_GROUND_1,
    &SLIME_CAVE,
];

/// Get map by ID
pub fn get_map_by_id(id: &str) -> Option<&'static MapDef> {
    ALL_MAPS.iter().find(|m| m.id == id).copied()
}

/// Dungeon definitions for progression
#[derive(Debug, Clone)]
pub struct DungeonDef {
    pub id: i32,
    pub map_id: &'static str,
    pub name_key: &'static str,
    pub level_req: i32,
    pub required_clears_for_next: i32,
}

pub const ALL_DUNGEONS: &[DungeonDef] = &[
    DungeonDef {
        id: 1,
        map_id: "hunting_ground_1",
        name_key: "dungeon.beginner_hunting",
        level_req: 1,
        required_clears_for_next: 10,
    },
    DungeonDef {
        id: 2,
        map_id: "slime_cave",
        name_key: "dungeon.slime_cave",
        level_req: 10,
        required_clears_for_next: 10,
    },
];
