use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use crate::shared::domain::{Direction, MonsterAIType, PlayerClass};
use crate::shared::domain::character::models::Player;
use crate::shared::domain::monster::{Monster, MonsterData, SpriteSize};
use crate::shared::domain::skill::models::Skill;
use crate::shared::domain::shared::models::Position;

const ISO_CHART_WIDTH: f32 = 64.0;
const ISO_CHART_HEIGHT: f32 = 32.0;

/// Projects logical grid coordinates (x, y) to screen isometric coordinates (x, y)
/// Darkages/Legend of Darkness style 2:1 ratio.
fn project_iso(grid_x: f32, grid_y: f32) -> Vec2 {
    let x = (grid_x - grid_y) * (ISO_CHART_WIDTH / 2.0);
    let y = (grid_x + grid_y) * -(ISO_CHART_HEIGHT / 2.0);
    Vec2::new(x, y)
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
    i18n: Res<I18nResource>,
    assets: Res<GameAssets>,
) {
    // ... existing tile spawning ...
    
    // Millres Layout (16x16)
    let layout = [
        "GGGGGGSSSSGGGGGG", // 0
        "GGWWGSSSSGGWWWGG", // 1
        "GGDWGSSSSGWDDWGG", // 2
        "GGGGGGSSSSGGGGGG", // 3
        "SSSSSSSSSSSSSSSS", // 4
        "SSSSSFFFSSSSSSSS", // 5
        "SSSSSFFFSSSSSSSS", // 6
        "SSSSSFFFSSSSSSSS", // 7
        "SSSSSSSSSSSSSSSS", // 8
        "GGGGGGSSSSGGGGGG", // 9
        "GWWGGSSSSGGWWWWG", // 10
        "GWDDGSSSSGWWDDWG", // 11
        "GGGGGGSSSSGGGGGG", // 12
        "GGGGGGSSSSGGGGGG", // 13
        "GGGGGGSSSSGGGGGG", // 14
        "GGGGGGSSSSGGGGGG", // 15
    ];

    for (y, row) in layout.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let tile_type = match char {
                'G' => TileType::Grass,
                'S' => TileType::Stone,
                'F' => TileType::Fountain,
                'W' => TileType::Wall,
                'D' => TileType::Door,
                _ => TileType::Grass,
            };

            let color = match tile_type {
                TileType::Grass => Color::srgb(0.12, 0.18, 0.12), // Deep murky green
                TileType::Stone => Color::srgb(0.22, 0.22, 0.24), // Cold grey stone
                TileType::Fountain => Color::srgb(0.25, 0.35, 0.45), // Dark mystical water
                TileType::Wall => Color::srgb(0.18, 0.15, 0.12), // Ancient dark wood/stone
                TileType::Door => Color::srgb(0.3, 0.15, 0.08), // Dark mahogany
            };

            let iso_pos = project_iso(x as f32, y as f32);
            
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(ISO_CHART_WIDTH, ISO_CHART_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(iso_pos.x, iso_pos.y, -iso_pos.y / 1000.0), // Y-Sorting base
                TileComponent { tile_type },
                GridPosition { x: x as i32, y: y as i32 },
            ));
        }
    }

    let spawn_pos = project_iso(8.0, 8.0);

    // Spawn player
    commands.spawn((
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
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
        AnimationState::default(),
        CombatState::default(),
        ActiveSkills::default(),
        CameraTarget,
    ));
    
    // Spawn some monsters
    let monster_positions = vec![
        (3, 3, "monsters.slime", 1),
        (3, 13, "monsters.slime", 1),
        (13, 3, "monsters.goblin", 3),
        (13, 13, "monsters.skeleton", 5),
    ];
    
    for (x, y, name, level) in monster_positions {
        spawn_monster(&mut commands, x, y, name, level, &i18n);
    }
    
    // Spawn NPCs (Shopkeeper and Innkeeper)
    spawn_npc(&mut commands, 2, 2, &i18n.t("ui.innkeeper"), InteractionType::NpcChat(i18n.t("ui.innkeeper_msg")));
    spawn_npc(&mut commands, 12, 11, &i18n.t("ui.shopkeeper"), InteractionType::NpcChat(i18n.t("ui.shopkeeper_msg")));

    // Spawn HUD
    spawn_hud(&mut commands, assets.ui_font.clone());
}

fn spawn_npc(commands: &mut Commands, x: i32, y: i32, name: &str, interaction: InteractionType) {
    let iso_pos = project_iso(x as f32, y as f32);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.2), // Yellowish for NPCs
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(iso_pos.x, iso_pos.y, 5.0),
        GridPosition { x, y },
        TargetGridPosition { x, y },
        Interactable {
            message: name.to_string(),
            interaction_type: interaction,
        },
        Facing::default(),
    ));
}

fn spawn_monster(commands: &mut Commands, grid_x: i32, grid_y: i32, name: &str, level: i32, i18n: &I18nResource) {
    let monster_data = MonsterData {
        id: 0,
        name: i18n.t(name),
        level,
        max_hp: 30 + level * 10,
        attack_min: 2 + level,
        attack_max: 5 + level,
        defense: level,
        exp_reward: 10 * level,
        gold_min: 5 * level,
        gold_max: 15 * level,
        ai_type: MonsterAIType::Aggressive,
        detection_range: 5.0, // In tiles
        attack_range: 1.0,    // In tiles
        move_speed: 1.0,
        sprite_path: "".to_string(),
        sprite_type: "slime".to_string(),
        sprite_size: SpriteSize::Small,
    };

    let monster = Monster::new(&monster_data, Position::new(grid_x as f64, grid_y as f64));
    let iso_pos = project_iso(grid_x as f32, grid_y as f32);

    commands.spawn((
        Sprite {
            color: MONSTER_COLOR,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(iso_pos.x, iso_pos.y, 5.0),
        MonsterComponent,
        monster,
        MonsterAI {
            ai_type: MonsterAIType::Aggressive,
            detection_range: 5.0,
            attack_range: 1.0,
            move_speed: 0.4, // Seconds per tile
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
        AnimationState::default(),
    ));
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

/// Smoothly interpolates entities between grid positions
pub fn character_grid_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut GridPosition,
        &mut TargetGridPosition,
        &mut MovementProgress,
        &mut Sprite,
    )>,
) {
    for (mut transform, mut grid_pos, target_pos, mut progress, _) in &mut query {
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

pub fn player_animation(
    time: Res<Time>,
    mut query: Query<(&GridPosition, &TargetGridPosition, &mut AnimationState, &mut Sprite), With<PlayerComponent>>,
) {
    for (grid_pos, target_pos, mut anim, mut sprite) in &mut query {
        let is_moving = grid_pos.x != target_pos.x || grid_pos.y != target_pos.y;
        
        if is_moving {
            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                anim.current_frame = (anim.current_frame + 1) % anim.frame_count;
            }
            
            // simple bobbing
            let _bob = (anim.current_frame as f32 * 0.1).sin() * 2.0;
            sprite.color = Color::srgb(0.3, 0.5, 0.8);
        } else {
            sprite.color = Color::srgb(0.3, 0.5, 0.8);
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
            
            match ai.ai_type {
                MonsterAIType::Aggressive => {
                    if distance < ai.detection_range && distance > ai.attack_range {
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
            if tile.tile_type == TileType::Door {
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
    i18n: Res<I18nResource>,
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
                let skill_name = i18n.t(&skill.name);
                
                // Check cooldown
                if let Some(skill_cd) = active_skills.skills.iter().find(|s| s.skill_id == skill.id) {
                    if !skill_cd.timer.finished() {
                        println!("⏳ {} {}", skill_name, i18n.t("ui.cooldown_msg"));
                        continue;
                    }
                }

                // Check MP
                if player.combat_stats.mp < skill.mp_cost {
                    println!("❌ {}", i18n.t("ui.not_enough_mp"));
                    continue;
                }

                // Execute skill
                println!("🔥 {} {}", i18n.t("ui.skill_activated"), skill_name);
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
                                monster.hp -= damage;
                                println!("💥 {}에게 {} {} 데미지!", monster.name, skill.name, damage);
                                if monster.hp <= 0 { commands.entity(entity).despawn(); }
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
