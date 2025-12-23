//! Game Resources

use bevy::prelude::*;
use std::collections::HashMap;
use crate::shared::domain::monster::MonsterData;
use crate::shared::data::{monsters, skills};

/// Game configuration
#[derive(Resource)]
pub struct GameConfig {
    #[allow(dead_code)]
    pub tile_size: f32,
    #[allow(dead_code)]
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
    
    // Sprite textures
    pub tile_atlas: Option<Handle<Image>>,
    pub buildings_atlas: Option<Handle<Image>>,
    
    // Character sprites (class_name -> gender -> handle)
    pub character_sprites: HashMap<String, HashMap<String, Handle<Image>>>,
    
    // Monster sprites (sprite_type -> handle)
    pub monster_sprites: HashMap<String, Handle<Image>>,
    
    // Decoration sprites
    pub torch_sprite: Option<Handle<Image>>,
}

impl GameAssets {
    /// Get character sprite for class and gender
    pub fn get_character_sprite(&self, class_name: &str, gender: &str) -> Option<Handle<Image>> {
        self.character_sprites
            .get(class_name)
            .and_then(|genders| genders.get(gender))
            .cloned()
    }
    
    /// Get monster sprite by type
    pub fn get_monster_sprite(&self, sprite_type: &str) -> Option<Handle<Image>> {
        self.monster_sprites.get(sprite_type).cloned()
    }
}

/// Selected character class in character creation
#[derive(Resource, Default)]
pub struct SelectedClass {
    pub class: Option<crate::shared::domain::PlayerClass>,
    pub gender: String,
    pub username: String,
}

/// Skill definitions from data module
#[derive(Resource)]
pub struct SkillData {
    pub skills: Vec<crate::shared::domain::skill::models::Skill>,
}

impl Default for SkillData {
    fn default() -> Self {
        // Load from static data
        let skills = skills::ALL_SKILLS.iter().map(|s| {
            crate::shared::domain::skill::models::Skill {
                id: s.id,
                name: s.name.to_string(),
                class_id: s.class_id,
                req_level: s.req_level,
                mp_cost: s.mp_cost,
                cooldown_ms: s.cooldown_ms,
                description: Some(s.description_key.to_string()),
                effect_type: Some(match s.effect_type {
                    skills::SkillEffectType::Damage => "damage".to_string(),
                    skills::SkillEffectType::Heal => "heal".to_string(),
                    skills::SkillEffectType::Buff => "buff".to_string(),
                    skills::SkillEffectType::Debuff => "debuff".to_string(),
                }),
                base_value: s.base_value,
                icon_path: Some(s.icon_path.to_string()),
            }
        }).collect();
        
        Self { skills }
    }
}

/// Monster definitions from data module
#[derive(Resource)]
pub struct MonsterDefinitions {
    pub definitions: HashMap<String, MonsterData>,
}

impl Default for MonsterDefinitions {
    fn default() -> Self {
        let mut definitions = HashMap::new();
        
        for monster_def in monsters::ALL_MONSTERS {
            let loot_drops: Vec<crate::shared::domain::monster::LootDrop> = 
                monsters::get_monster_drops(monster_def.id)
                    .into_iter()
                    .map(|d| crate::shared::domain::monster::LootDrop {
                        item_id: d.item_id,
                        probability: d.probability,
                        min_quantity: d.min_quantity,
                        max_quantity: d.max_quantity,
                    })
                    .collect();
            
            let monster_data = MonsterData {
                id: monster_def.id,
                name: monster_def.name.to_string(),
                level: monster_def.level,
                max_hp: monster_def.hp_max,
                attack_min: monster_def.attack_min,
                attack_max: monster_def.attack_max,
                defense: monster_def.defense,
                exp_reward: monster_def.exp_reward,
                gold_min: monster_def.gold_min,
                gold_max: monster_def.gold_max,
                ai_type: monster_def.ai_type,
                detection_range: monster_def.detection_range,
                attack_range: monster_def.attack_range,
                move_speed: monster_def.move_speed,
                sprite_path: monster_def.sprite_path(),
                sprite_type: monster_def.sprite_type.to_string(),
                sprite_size: monster_def.sprite_size,
                description: monster_def.description_key.to_string(),
                metadata: None,
                loot_table: loot_drops,
            };
            
            definitions.insert(monster_def.name.to_string(), monster_data);
        }
        
        Self { definitions }
    }
}

/// Text resource for UI strings
/// Currently English only. Ready for future i18n expansion.
#[derive(Resource)]
pub struct TextResource;

impl Default for TextResource {
    fn default() -> Self {
        Self
    }
}

impl TextResource {
    /// Get text by key. Returns English text directly.
    /// When i18n is needed, this will load from language packs.
    pub fn get(&self, key: &str) -> &'static str {
        match key {
            // Main Menu
            "ui.title" => "Legend of Darkness",
            "ui.subtitle" => "Dark Fantasy RPG",
            "ui.start_game" => "Start Game",
            "ui.quit" => "Quit",
            
            // Character Select
            "ui.character_select" => "Select Your Class",
            "ui.warrior" => "Warrior",
            "ui.rogue" => "Rogue",
            "ui.mage" => "Mage",
            "ui.cleric" => "Cleric",
            "ui.martial_artist" => "Martial Artist",
            "ui.select" => "Confirm",
            "ui.back" => "Back",
            
            // HUD
            "ui.level" => "Level",
            "ui.hp" => "HP",
            "ui.mp" => "MP",
            "ui.exp" => "EXP",
            "ui.gold" => "Gold",
            
            // Classes
            "class.warrior" => "Warrior",
            "class.warrior.desc" => "A powerful melee fighter with high strength and defense.",
            "class.rogue" => "Rogue",
            "class.rogue.desc" => "A swift assassin specializing in critical strikes.",
            "class.mage" => "Mage",
            "class.mage.desc" => "A master of arcane magic with devastating spells.",
            "class.cleric" => "Cleric",
            "class.cleric.desc" => "A holy healer who supports allies.",
            "class.martial_artist" => "Martial Artist",
            "class.martial_artist.desc" => "A balanced fighter with fast combos.",
            
            // Default: return key as unknown text marker
            _ => "[???]",
        }
    }
}



/// Sprite atlas definitions for texture atlases
#[derive(Resource, Default)]
pub struct SpriteAtlases {
    #[allow(dead_code)]
    pub monster_atlases: HashMap<String, Handle<TextureAtlasLayout>>,
    #[allow(dead_code)]
    pub character_atlases: HashMap<String, Handle<TextureAtlasLayout>>,
    #[allow(dead_code)]
    pub tile_atlas: Option<Handle<TextureAtlasLayout>>,
}
