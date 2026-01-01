use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use crate::shared::domain::{Direction, MonsterAIType, PlayerClass};
use crate::shared::domain::character::models::Player;
use crate::shared::domain::monster::Monster;
use crate::shared::domain::skill::models::Skill;
use crate::shared::domain::shared::models::Position;
use crate::shared::constants::*;


/// Projects logical grid coordinates (x, y) to screen isometric coordinates (x, y)
/// Darkages/Legend of Darkness style 2:1 ratio.
fn project_iso(grid_x: f32, grid_y: f32) -> Vec2 {
    let iso_x = (grid_x - grid_y) * (TILE_WIDTH / 2.0);
    let iso_y = (grid_x + grid_y) * (TILE_HEIGHT / 2.0);
    Vec2::new(iso_x, -iso_y)
}

// ============ Color Constants ============

#[allow(dead_code)]
const GRASS_COLOR: Color = Color::srgb(0.2, 0.35, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.5, 0.8);
const MONSTER_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
const HP_BAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);
const HP_BAR_FG: Color = Color::srgb(0.8, 0.1, 0.1);
const MP_BAR_FG: Color = Color::srgb(0.1, 0.3, 0.8);

// ============ World Setup ============

pub fn spawn_game_world(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    monster_defs: Res<MonsterDefinitions>,
    manifests: Res<Assets<crate::shared::domain::sprite::SpriteManifest>>,
) {
    // =============================================
    // Spawn Tiles using tile atlas or fallback colors
    // =============================================
    
    // Simple 16x16 grid layout (planned: load from map data)
    const MAP_SIZE: usize = 16;
    let map_layout: [[char; MAP_SIZE]; MAP_SIZE] = [
        ['G','G','G','G','G','G','S','S','S','S','G','G','G','G','G','G'],
        ['G','G','B','W','G','S','S','S','S','S','G','G','B','W','W','G'],
        ['G','G','D','W','G','S','S','S','S','S','G','W','D','D','W','G'],
        ['G','G','G','G','G','G','S','S','S','S','G','G','G','G','G','G'],
        ['S','S','S','S','S','S','S','S','S','S','S','S','S','S','S','S'],
        ['S','S','S','S','S','F','F','F','S','S','S','S','S','S','S','S'],
        ['S','S','S','S','S','F','F','F','S','S','S','S','S','S','S','S'],
        ['S','S','S','S','S','F','F','F','S','S','S','S','S','S','S','S'],
        ['S','S','S','S','S','S','S','S','S','S','S','S','S','S','S','S'],
        ['G','G','G','G','G','G','S','S','S','S','G','G','G','G','G','G'],
        ['G','B','W','G','G','S','S','S','S','S','G','G','B','W','W','G'],
        ['G','B','D','D','G','S','S','S','S','S','G','W','W','D','D','G'],
        ['G','G','G','G','G','G','S','S','S','S','G','G','G','G','G','G'],
        ['G','G','G','G','G','G','S','S','S','S','G','G','G','G','G','G'],
        ['T','T','T','T','G','G','S','S','S','S','G','G','T','T','T','T'],
        ['T','T','T','T','G','G','S','S','S','S','G','G','T','T','T','T'],
    ];
    
    for (y, row) in map_layout.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            let tile_type = match char {
                'G' => RenderTileType::Grass,
                'S' => RenderTileType::Stone,
                'F' => RenderTileType::Fountain,
                'W' | 'X' => RenderTileType::Wall,
                'D' => RenderTileType::Door,
                'B' => RenderTileType::Wall, // Building
                'T' => RenderTileType::Wall, // Tree (non-walkable)
                _ => RenderTileType::Grass,
            };

            // Use texture if available, otherwise fallback to colors
            let color = match tile_type {
                RenderTileType::Grass => Color::srgb(0.15, 0.22, 0.12),    // Dark forest green
                RenderTileType::Stone => Color::srgb(0.25, 0.25, 0.28),    // Cold grey cobblestone
                RenderTileType::Fountain => Color::srgb(0.2, 0.3, 0.4),    // Dark mystical water
                RenderTileType::Wall => Color::srgb(0.2, 0.18, 0.15),      // Ancient dark stone
                RenderTileType::Door => Color::srgb(0.35, 0.2, 0.1),       // Dark wood door
            };

            let iso_pos = project_iso(x as f32, y as f32);
            
            // Try to use tile texture, fallback to colored sprite
            let sprite = if let (Some(image), Some(layout)) = (assets.tile_atlas.clone(), assets.tile_atlas_layout.clone()) {
                let index = match tile_type {
                    RenderTileType::Grass => 0,
                    RenderTileType::Stone => 4,
                    RenderTileType::Fountain => 1,
                    RenderTileType::Wall => 5,
                    RenderTileType::Door => 2,
                };
                
                Sprite {
                    image,
                    texture_atlas: Some(TextureAtlas {
                        layout,
                        index,
                    }),
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                    ..default()
                }
            } else {
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                    ..default()
                }
            };
            
            commands.spawn((
                sprite,
                Transform::from_xyz(iso_pos.x, iso_pos.y, -iso_pos.y / 1000.0),
                TileComponent { tile_type },
                GridPosition { x: x as i32, y: y as i32 },
            ));
        }
    }

    let spawn_pos = project_iso(8.0, 8.0);

    // =============================================  
    // Spawn Player with sprite or fallback color
    // =============================================
    // =============================================  
    // Spawn Player with sprite or fallback color
    // =============================================
    let manifest_id = "warrior_male";
    let player_sprite = if let Some(sprite_handle) = assets.get_character_sprite("warrior", "male") {
        let mut s = Sprite {
            image: sprite_handle,
            ..default()
        };
        
        // Pre-set initial rect if manifest is available to avoid "box flashing"
        if let Some(manifest_handle) = assets.manifests.get(manifest_id) {
            if let Some(manifest) = manifests.get(manifest_handle) {
                let (index, flip) = manifest.get_frame_index(crate::shared::domain::sprite::AnimationState::Idle, crate::shared::domain::sprite::SpriteDirection::Down, 0);
                let (rx, ry, rw, rh) = manifest.layout.get_frame_rect(index);
                s.rect = Some(Rect::new(rx as f32, ry as f32, (rx + rw) as f32, (ry + rh) as f32));
                s.custom_size = Some(Vec2::new(rw as f32, rh as f32));
                s.flip_x = flip;
                s.anchor = bevy::sprite::Anchor::BottomCenter;
            }
        }
        s
    } else {
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(Vec2::new(CHARACTER_RENDER_WIDTH, CHARACTER_RENDER_HEIGHT)),
            ..default()
        }
    };
    
    commands.spawn((
        player_sprite,
        Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
        PlayerComponent,
        Player::new("Player".to_string(), PlayerClass::Warrior),
        Facing::default(),
        GridPosition { x: 8, y: 8 },
        TargetGridPosition { x: 8, y: 8 },
        MovementProgress {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            start_pos: Vec2::new(8.0, 8.0),
        },
        super::animation::SpriteAnimator {
            manifest: assets.manifests.get(manifest_id).cloned(),
            ..default()
        },
        CombatState::default(),
        ActiveSkills::default(),
        CameraTarget,
    ));
    
    // =============================================
    // Spawn Monsters with sprites
    // =============================================
    let monster_positions = vec![
        (3, 3, "Giant Rat"),
        (3, 13, "Vampire Bat"),
        (13, 3, "Slime"), 
        (13, 13, "Slime"),
    ];
    
    for (x, y, name) in monster_positions {
        spawn_monster(&mut commands, x, y, name, &monster_defs, &assets, &manifests);
    }
    
    // =============================================
    // Spawn NPCs
    // =============================================
    spawn_npc(&mut commands, 2, 2, "Innkeeper", InteractionType::NpcChat("Welcome! Rest here to recover.".to_string()));
    spawn_npc(&mut commands, 12, 11, "Shopkeeper", InteractionType::NpcChat("Buy and sell items.".to_string()));

    // =============================================
    // Spawn HUD
    // =============================================
    spawn_hud(&mut commands, assets.ui_font.clone());
    
    println!("🗺️ Game world spawned with {} tiles", MAP_SIZE * MAP_SIZE);
}

fn spawn_npc(commands: &mut Commands, x: i32, y: i32, name: &str, interaction: InteractionType) {
    let iso_pos = project_iso(x as f32, y as f32);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.2), // Yellowish for NPCs
            custom_size: Some(Vec2::new(CHARACTER_RENDER_WIDTH, CHARACTER_RENDER_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(iso_pos.x, iso_pos.y, Z_LAYER_ENTITY_BASE),
        GridPosition { x, y },
        TargetGridPosition { x, y },
        Interactable {
            message: name.to_string(),
            interaction_type: interaction,
        },
        Facing::default(),
    ));
}

fn spawn_monster(
    commands: &mut Commands, 
    grid_x: i32, 
    grid_y: i32, 
    name_key: &str, 
    monster_defs: &MonsterDefinitions,
    assets: &GameAssets,
    manifests: &Assets<crate::shared::domain::sprite::SpriteManifest>,
) {
    // Look up monster definition
    let Some(data) = monster_defs.definitions.get(name_key) else {
        println!("❌ Failed to spawn monster: {} (Not found in data module)", name_key);
        return;
    };
    
    let monster = Monster::new(data, Position::new(grid_x as f64, grid_y as f64));
    let iso_pos = project_iso(grid_x as f32, grid_y as f32);
    
    // Get sprite size based on monster level/size
    let sprite_size = match data.sprite_size {
        crate::shared::domain::monster::SpriteSize::Small => Vec2::new(MONSTER_SPRITE_SMALL * MONSTER_RENDER_SCALE, MONSTER_SPRITE_SMALL * MONSTER_RENDER_SCALE),
        crate::shared::domain::monster::SpriteSize::Medium => Vec2::new(MONSTER_SPRITE_MEDIUM * MONSTER_RENDER_SCALE, MONSTER_SPRITE_MEDIUM * MONSTER_RENDER_SCALE),
        crate::shared::domain::monster::SpriteSize::Large => Vec2::new(MONSTER_SPRITE_LARGE * MONSTER_RENDER_SCALE, MONSTER_SPRITE_LARGE * MONSTER_RENDER_SCALE),
        crate::shared::domain::monster::SpriteSize::Boss => Vec2::new(MONSTER_SPRITE_BOSS * MONSTER_RENDER_SCALE, MONSTER_SPRITE_BOSS * MONSTER_RENDER_SCALE),
    };
    
    // Try to get monster sprite from assets
    let sprite = if let Some(sprite_handle) = assets.get_monster_sprite(&data.sprite_type) {
        let mut s = Sprite {
            image: sprite_handle,
            ..default()
        };
        
        // Pre-set initial rect if manifest is available
        if let Some(manifest_handle) = assets.manifests.get(&data.sprite_type) {
            if let Some(manifest) = manifests.get(manifest_handle) {
                let (index, flip) = manifest.get_frame_index(crate::shared::domain::sprite::AnimationState::Idle, crate::shared::domain::sprite::SpriteDirection::Down, 0);
                let (rx, ry, rw, rh) = manifest.layout.get_frame_rect(index);
                s.rect = Some(Rect::new(rx as f32, ry as f32, (rx + rw) as f32, (ry + rh) as f32));
                s.custom_size = Some(Vec2::new(rw as f32, rh as f32));
                s.flip_x = flip;
                s.anchor = bevy::sprite::Anchor::BottomCenter;
            }
        }
        s
    } else {
        // Fallback to colored rectangle...
        let color = match data.sprite_type.as_str() {
            "rat" => Color::srgb(0.5, 0.4, 0.3),      // Brownish
            "bat" => Color::srgb(0.3, 0.2, 0.4),      // Dark purple
            "slime" => Color::srgb(0.2, 0.7, 0.3),    // Green slime
            "wolf" => Color::srgb(0.4, 0.4, 0.45),    // Grey
            "skeleton" => Color::srgb(0.8, 0.8, 0.75), // Bone white
            "goblin" => Color::srgb(0.3, 0.5, 0.2),   // Green
            "ghost" => Color::srgba(0.7, 0.7, 0.9, 0.6), // Translucent blue
            "dragon" => Color::srgb(0.8, 0.2, 0.1),   // Red
            _ => MONSTER_COLOR,
        };
        Sprite {
            color,
            custom_size: Some(sprite_size),
            ..default()
        }
    };

    commands.spawn((
        sprite,
        Transform::from_xyz(iso_pos.x, iso_pos.y, Z_LAYER_ENTITY_BASE + iso_pos.y.abs() / 1000.0),
        MonsterComponent,
        monster,
        MonsterAI {
            ai_type: data.ai_type,
            detection_range: data.detection_range as f32,
            attack_range: data.attack_range as f32,
            move_speed: data.move_speed as f32,
            target: None,
            spawn_position: Vec2::new(grid_x as f32, grid_y as f32),
        },
        GridPosition { x: grid_x, y: grid_y },
        TargetGridPosition { x: grid_x, y: grid_y },
        MovementProgress {
            timer: Timer::from_seconds(0.4, TimerMode::Once),
            start_pos: Vec2::new(grid_x as f32, grid_y as f32),
        },
        Facing::default(),
        super::animation::SpriteAnimator {
            manifest: assets.manifests.get(&data.sprite_type).cloned(),
            ..default()
        },
    ));
    
    println!("👾 Spawned {} at ({}, {})", name_key, grid_x, grid_y);
}

fn spawn_hud(commands: &mut Commands, font: Handle<Font>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        super::components::HudUI,
    ))
    .with_children(|parent| {
        // Level text
        parent.spawn((
            Text::new("Lv.1"),
            TextFont {
                font: font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            LevelText,
        ));
        
        // HP Bar container
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(HP_BAR_BG),
        ))
        .with_children(|bar| {
            bar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(HP_BAR_FG),
                HpBar,
            ));
        });
        
        // MP Bar container
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(15.0),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(HP_BAR_BG),
        ))
        .with_children(|bar| {
            bar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(MP_BAR_FG),
                MpBar,
            ));
        });
        
        // Gold text
        parent.spawn((
            Text::new("💰 100"),
            TextFont {
                font: font,
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.855, 0.647, 0.125)),
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            GoldText,
        ));
    });
}

// ============ Player Systems ============

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&GridPosition, &mut TargetGridPosition, &mut Facing), With<PlayerComponent>>,
) {
    if let Ok((grid_pos, mut target_pos, mut facing)) = query.get_single_mut() {
        // Only allow new movement if we have reached the target
        if grid_pos.x == target_pos.x && grid_pos.y == target_pos.y {
            let mut move_dir = IVec2::ZERO;
            
            if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
                move_dir.y -= 1;
                facing.direction = Direction::Up;
            } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
                move_dir.y += 1;
                facing.direction = Direction::Down;
            } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
                move_dir.x -= 1;
                facing.direction = Direction::Left;
            } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
                move_dir.x += 1;
                facing.direction = Direction::Right;
            }

            if move_dir != IVec2::ZERO {
                target_pos.x = grid_pos.x + move_dir.x;
                target_pos.y = grid_pos.y + move_dir.y;
            }
        }
    }
}

/// Syncs logical state (movement, facing) to animation state
pub fn sync_character_animation(
    mut query: Query<(&GridPosition, &TargetGridPosition, &Facing, &mut super::animation::SpriteAnimator)>,
) {
    for (grid_pos, target_pos, facing, mut anim) in &mut query {
        let is_moving = grid_pos.x != target_pos.x || grid_pos.y != target_pos.y;
        
        let new_state = if is_moving {
            crate::shared::domain::sprite::AnimationState::Walk
        } else {
            crate::shared::domain::sprite::AnimationState::Idle
        };

        if anim.state != new_state {
            anim.state = new_state;
            anim.current_frame = 0;
            anim.timer.reset();
        }

        // Sync direction
        let dir = match facing.direction {
            crate::shared::domain::Direction::Down => crate::shared::domain::sprite::SpriteDirection::Down,
            crate::shared::domain::Direction::Up => crate::shared::domain::sprite::SpriteDirection::Up,
            crate::shared::domain::Direction::Left => crate::shared::domain::sprite::SpriteDirection::Left,
            crate::shared::domain::Direction::Right => crate::shared::domain::sprite::SpriteDirection::Right,
        };
        
        anim.direction = dir;
    }
}

/// Smoothly interpolates entities between grid positions
pub fn character_grid_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut GridPosition,
        &mut TargetGridPosition,
        &mut MovementProgress,
    )>,
) {
    for (mut transform, mut grid_pos, target_pos, mut progress) in &mut query {
        if grid_pos.x != target_pos.x || grid_pos.y != target_pos.y {
            progress.timer.tick(time.delta());
            
            let t = progress.timer.fraction();
            let start_iso = project_iso(grid_pos.x as f32, grid_pos.y as f32);
            let end_iso = project_iso(target_pos.x as f32, target_pos.y as f32);
            
            let current_pos = start_iso.lerp(end_iso, t);
            transform.translation.x = current_pos.x;
            transform.translation.y = current_pos.y;
            
            // Y-Sorting: Z depends on Y
            transform.translation.z = -current_pos.y / 1000.0 + 10.0; // +10 offset for characters

            if progress.timer.finished() {
                grid_pos.x = target_pos.x;
                grid_pos.y = target_pos.y;
                progress.timer.reset();
                progress.start_pos = Vec2::new(grid_pos.x as f32, grid_pos.y as f32);
            }
        } else {
            // Snap to grid just in case
            let iso = project_iso(grid_pos.x as f32, grid_pos.y as f32);
            transform.translation.x = iso.x;
            transform.translation.y = iso.y;
            transform.translation.z = -iso.y / 1000.0 + 10.0;
        }
    }
}

pub fn camera_follow(
    player_query: Query<&Transform, (With<CameraTarget>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Smooth camera follow
            let target = player_transform.translation;
            let current = camera_transform.translation;
            
            camera_transform.translation = current.lerp(
                Vec3::new(target.x, target.y, current.z),
                0.1,
            );
        }
    }
}

// ============ Monster Systems ============

pub fn monster_ai(
    player_query: Query<&GridPosition, With<PlayerComponent>>,
    mut monster_query: Query<
        (&GridPosition, &mut TargetGridPosition, &MonsterAI, &mut Facing),
        (With<MonsterComponent>, Without<PlayerComponent>),
    >,
) {
    let Ok(player_grid_pos) = player_query.get_single() else {
        return;
    };
    
    for (grid_pos, mut target_pos, ai, mut facing) in &mut monster_query {
        // Only move if currently at a grid position
        if grid_pos.x == target_pos.x && grid_pos.y == target_pos.y {
            let dx = player_grid_pos.x - grid_pos.x;
            let dy = player_grid_pos.y - grid_pos.y;
            let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
            
            // Convert pixel-based range to grid-based range (rough estimation)
            let detection_grid_range = ai.detection_range as f64 / GRID_UNIT as f64;
            let attack_grid_range = 1.1; // Close range
            
            match ai.ai_type {
                MonsterAIType::Aggressive => {
                    if distance < detection_grid_range as f32 && distance > attack_grid_range {
                        // Move towards player (simple pathfinding)
                        let move_x = if dx.abs() > dy.abs() { dx.signum() } else { 0 };
                        let move_y = if dx.abs() <= dy.abs() { dy.signum() } else { 0 };
                        
                        target_pos.x = grid_pos.x + move_x;
                        target_pos.y = grid_pos.y + move_y;

                        // Set facing
                        if move_x > 0 { facing.direction = Direction::Right; }
                        else if move_x < 0 { facing.direction = Direction::Left; }
                        else if move_y > 0 { facing.direction = Direction::Down; }
                        else if move_y < 0 { facing.direction = Direction::Up; }
                    } else if distance > ai.detection_range {
                        // Return to spawn or wander (simple return)
                        let sx = ai.spawn_position.x as i32 - grid_pos.x;
                        let sy = ai.spawn_position.y as i32 - grid_pos.y;
                        if sx != 0 || sy != 0 {
                            target_pos.x = grid_pos.x + sx.signum();
                            target_pos.y = grid_pos.y + sy.signum();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// ============ Combat System ============

pub fn interaction_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<(&GridPosition, &Facing), With<PlayerComponent>>,
    interactable_query: Query<(&GridPosition, &Interactable)>,
    tile_query: Query<(&GridPosition, &TileComponent)>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) && !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }

    let Ok((player_pos, facing)) = player_query.get_single() else { return; };
    
    // Check tile in front of player
    let (dx, dy) = match facing.direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };
    
    let target_x = player_pos.x + dx;
    let target_y = player_pos.y + dy;

    // 1. Check for NPCs/Interactables in front
    for (npc_pos, interactable) in &interactable_query {
        if npc_pos.x == target_x && npc_pos.y == target_y {
            match &interactable.interaction_type {
                InteractionType::NpcChat(msg) => {
                    println!("💬 {}: \"{}\"", interactable.message, msg);
                }
                _ => {}
            }
            return;
        }
    }

    // 2. Check for Doors/Portals at current or target position
    for (tile_pos, tile) in &tile_query {
        if tile_pos.x == target_x && tile_pos.y == target_y {
            if tile.tile_type == RenderTileType::Door {
                println!("🚪 건물 안으로 들어갑니다... (맵 전환 보류)");
                return;
            }
        }
    }
}

pub fn skill_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&GridPosition, &Facing, &mut Player, &mut ActiveSkills), With<PlayerComponent>>,
    mut monster_query: Query<(Entity, &GridPosition, &mut Monster), With<MonsterComponent>>,
    mut commands: Commands,
    skill_data: Res<SkillData>,
) {
    let Ok((player_pos, facing, mut player, mut active_skills)) = player_query.get_single_mut() else { return; };

    // Update cooldown timers
    for skill_cd in &mut active_skills.skills {
        skill_cd.timer.tick(time.delta());
    }

    // Get skills for player's class
    let player_class_id = player.class.id();
    let available_skills: Vec<&Skill> = skill_data.skills.iter()
        .filter(|s| (s.class_id == Some(player_class_id) || s.class_id.is_none()) && s.req_level <= player.level)
        .collect();

    let keys = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5];
    
    for (i, key) in keys.iter().enumerate() {
        if keyboard_input.just_pressed(*key) {
            if let Some(skill) = available_skills.get(i) {
                let skill_name = &skill.name;
                
                // Check cooldown
                if let Some(skill_cd) = active_skills.skills.iter().find(|s| s.skill_id == skill.id) {
                    if !skill_cd.timer.finished() {
                        println!("⏳ {} is on cooldown", skill_name);
                        continue;
                    }
                }

                // Check MP
                if player.combat_stats.mp < skill.mp_cost {
                    println!("❌ Not enough MP for {}", skill_name);
                    continue;
                }

                // Execute skill
                println!("🔥 Skill activated: {}", skill_name);
                player.combat_stats.mp -= skill.mp_cost;

                // Set cooldown
                if let Some(skill_cd) = active_skills.skills.iter_mut().find(|s| s.skill_id == skill.id) {
                    skill_cd.timer.set_duration(std::time::Duration::from_millis(skill.cooldown_ms as u64));
                    skill_cd.timer.reset();
                } else {
                    active_skills.skills.push(SkillCooldown {
                        skill_id: skill.id,
                        timer: Timer::new(std::time::Duration::from_millis(skill.cooldown_ms as u64), TimerMode::Once),
                    });
                }

                // Skill Effect handling
                let (dx, dy) = match facing.direction {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };

                let tx = player_pos.x + dx;
                let ty = player_pos.y + dy;

                match skill.effect_type.as_deref() {
                    Some("damage") => {
                        for (entity, m_pos, mut monster) in &mut monster_query {
                            if m_pos.x == tx && m_pos.y == ty {
                                let damage = skill.base_value + (player.combat_stats.attack_max / 2);
                                monster.take_damage(damage);
                                println!("💥 {} took {} damage from {}!", monster.name, damage, skill_name);
                                
                                if monster.is_dead() { 
                                    // Handle Loot and Rewards
                                    let (gold_reward, _item_rewards) = monster.calculate_loot();
                                    player.gold += gold_reward as i64;
                                    player.exp += monster.exp_reward as i64;
                                    
                                    println!("💰 +{} Gold! (Total: {})", gold_reward, player.gold);
                                    println!("📈 +{} EXP! (Total: {})", monster.exp_reward, player.exp);
                                    
                                    // Level Up Check
                                    if player.exp >= player.exp_to_next_level {
                                        player.level += 1;
                                        player.exp -= player.exp_to_next_level;
                                        player.exp_to_next_level += 100; // Simplified scale
                                        println!("🎉 레벨 업! 현재 레벨: {}", player.level);
                                    }

                                    commands.entity(entity).despawn(); 
                                }
                                break;
                            }
                        }
                    }
                    Some("heal") => {
                        let heal_amt = skill.base_value;
                        player.heal(heal_amt);
                        println!("✨ {} 회복! (+{})", skill.name, heal_amt);
                    }
                    _ => {
                        println!("ℹ️ {} 스킬이 사용되었습니다. (효과 미구현)", skill.name);
                    }
                }
            }
        }
    }
}

// ============ Cleanup ============

pub fn cleanup_game_world(
    mut commands: Commands,
    tiles: Query<Entity, With<TileComponent>>,
    players: Query<Entity, With<PlayerComponent>>,
    monsters: Query<Entity, With<MonsterComponent>>,
    hud: Query<Entity, With<super::components::HudUI>>,
) {
    for entity in tiles.iter().chain(players.iter()).chain(monsters.iter()).chain(hud.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}
