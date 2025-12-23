//! Game World Systems - Player, Monsters, Combat

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use crate::shared::domain::{Direction, PlayerClass, MonsterAIType};

// ============ Color Constants ============

const GRASS_COLOR: Color = Color::srgb(0.2, 0.35, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.5, 0.8);
const MONSTER_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
const HP_BAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);
const HP_BAR_FG: Color = Color::srgb(0.8, 0.1, 0.1);
const MP_BAR_FG: Color = Color::srgb(0.1, 0.3, 0.8);

// ============ World Setup ============

pub fn spawn_game_world(
    mut commands: Commands,
    selected_class: Res<SelectedClass>,
    config: Res<GameConfig>,
) {
    let tile_size = config.tile_size;
    let map_width = 30;
    let map_height = 20;
    
    // Spawn ground tiles
    for x in 0..map_width {
        for y in 0..map_height {
            commands.spawn((
                Sprite {
                    color: GRASS_COLOR,
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * tile_size - (map_width as f32 * tile_size / 2.0),
                    y as f32 * tile_size - (map_height as f32 * tile_size / 2.0),
                    0.0,
                ),
                Tile {
                    tile_type: TileType::Grass,
                    x,
                    y,
                },
            ));
        }
    }
    
    // Spawn player
    let player_class = selected_class.class.unwrap_or(PlayerClass::Warrior);
    commands.spawn((
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        PlayerComponent,
        PlayerStats {
            class: player_class,
            ..default()
        },
        Facing::default(),
        Velocity::default(),
        AnimationState::default(),
        CombatState::default(),
        CameraTarget,
    ));
    
    // Spawn some monsters
    let monster_positions = vec![
        (200.0, 100.0, "슬라임", 1),
        (-150.0, 200.0, "슬라임", 1),
        (300.0, -100.0, "고블린", 3),
        (-250.0, -150.0, "스켈레톤", 5),
    ];
    
    for (x, y, name, level) in monster_positions {
        spawn_monster(&mut commands, Vec2::new(x, y), name, level);
    }
    
    // Spawn HUD
    spawn_hud(&mut commands);
}

fn spawn_monster(commands: &mut Commands, position: Vec2, name: &str, level: i32) {
    let base_hp = 30 + level * 10;
    
    commands.spawn((
        Sprite {
            color: MONSTER_COLOR,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 5.0),
        MonsterComponent {
            monster_id: 1,
            name: name.to_string(),
        },
        MonsterStats {
            level,
            hp: base_hp,
            max_hp: base_hp,
            attack_min: 3 + level,
            attack_max: 6 + level * 2,
            defense: 2 + level,
            exp_reward: 10 * level,
            gold_min: 5 * level,
            gold_max: 15 * level,
        },
        MonsterAI {
            ai_type: MonsterAIType::Aggressive,
            detection_range: 150.0,
            attack_range: 40.0,
            move_speed: 50.0,
            target: None,
            spawn_position: position,
        },
        Facing::default(),
        Velocity::default(),
        AnimationState::default(),
    ));
}

fn spawn_hud(commands: &mut Commands) {
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
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Facing), With<PlayerComponent>>,
) {
    if let Ok((mut transform, mut velocity, mut facing)) = query.get_single_mut() {
        let mut direction = Vec2::ZERO;
        
        // WASD and Arrow keys
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
            facing.direction = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
            facing.direction = Direction::Down;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
            facing.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
            facing.direction = Direction::Right;
        }
        
        // Normalize for diagonal movement
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }
        
        velocity.x = direction.x * config.player_speed;
        velocity.y = direction.y * config.player_speed;
        
        // Apply movement
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn player_animation(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut AnimationState, &mut Sprite), With<PlayerComponent>>,
) {
    for (velocity, mut anim, mut sprite) in &mut query {
        let is_moving = velocity.x.abs() > 0.1 || velocity.y.abs() > 0.1;
        
        if is_moving {
            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                anim.current_frame = (anim.current_frame + 1) % anim.frame_count;
            }
            
            // Simple animation: slightly change color based on frame
            let brightness = 0.3 + (anim.current_frame as f32 * 0.05);
            sprite.color = Color::srgb(brightness, 0.5, 0.8);
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
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<PlayerComponent>>,
    mut monster_query: Query<
        (&mut Transform, &mut MonsterAI, &mut Velocity, &mut Facing),
        (With<MonsterComponent>, Without<PlayerComponent>),
    >,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single() else {
        return;
    };
    
    let player_pos = player_transform.translation.truncate();
    
    for (mut transform, mut ai, mut velocity, mut facing) in &mut monster_query {
        let monster_pos = transform.translation.truncate();
        let distance = monster_pos.distance(player_pos);
        
        match ai.ai_type {
            MonsterAIType::Aggressive => {
                if distance < ai.detection_range && distance > ai.attack_range {
                    // Move towards player
                    let direction = (player_pos - monster_pos).normalize();
                    velocity.x = direction.x * ai.move_speed;
                    velocity.y = direction.y * ai.move_speed;
                    
                    // Set facing direction
                    if direction.x.abs() > direction.y.abs() {
                        facing.direction = if direction.x > 0.0 { Direction::Right } else { Direction::Left };
                    } else {
                        facing.direction = if direction.y > 0.0 { Direction::Up } else { Direction::Down };
                    }
                    
                    ai.target = Some(player_entity);
                } else if distance <= ai.attack_range {
                    // Stop and attack
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                } else {
                    // Return to spawn
                    let spawn_distance = monster_pos.distance(ai.spawn_position);
                    if spawn_distance > 10.0 {
                        let direction = (ai.spawn_position - monster_pos).normalize();
                        velocity.x = direction.x * ai.move_speed * 0.5;
                        velocity.y = direction.y * ai.move_speed * 0.5;
                    } else {
                        velocity.x = 0.0;
                        velocity.y = 0.0;
                    }
                    ai.target = None;
                }
            }
            MonsterAIType::Passive | MonsterAIType::Defensive => {
                // Passive monsters don't move
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
        }
        
        // Apply movement
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

// ============ Combat System ============

pub fn combat_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut PlayerStats, &mut CombatState), With<PlayerComponent>>,
    mut monster_query: Query<
        (Entity, &Transform, &mut MonsterStats, &MonsterComponent),
        Without<PlayerComponent>,
    >,
    mut commands: Commands,
) {
    let Ok((player_transform, mut player_stats, mut combat_state)) = player_query.get_single_mut() else {
        return;
    };
    
    // Update attack cooldown
    combat_state.attack_timer.tick(time.delta());
    
    // Space to attack
    if keyboard_input.just_pressed(KeyCode::Space) && combat_state.attack_timer.finished() {
        combat_state.is_attacking = true;
        combat_state.attack_timer.reset();
        
        let player_pos = player_transform.translation.truncate();
        
        // Find nearest monster in attack range
        let attack_range = 60.0;
        let mut closest_monster: Option<(Entity, f32)> = None;
        
        for (entity, monster_transform, _, _) in &monster_query {
            let distance = player_pos.distance(monster_transform.translation.truncate());
            if distance < attack_range {
                if closest_monster.is_none() || distance < closest_monster.unwrap().1 {
                    closest_monster = Some((entity, distance));
                }
            }
        }
        
        // Attack the closest monster
        if let Some((target_entity, _)) = closest_monster {
            if let Ok((_, _, mut monster_stats, monster_component)) = monster_query.get_mut(target_entity) {
                let damage = player_stats.attack.max(1);
                monster_stats.hp -= damage;
                
                println!("⚔️ {}에게 {} 데미지!", monster_component.name, damage);
                
                if monster_stats.hp <= 0 {
                    // Monster died
                    let exp = monster_stats.exp_reward;
                    let gold = rand::random::<i32>().abs() % (monster_stats.gold_max - monster_stats.gold_min + 1) + monster_stats.gold_min;
                    
                    player_stats.exp += exp as i64;
                    player_stats.gold += gold as i64;
                    
                    // Level up check
                    while player_stats.exp >= player_stats.exp_to_next {
                        player_stats.exp -= player_stats.exp_to_next;
                        player_stats.level += 1;
                        player_stats.exp_to_next = (100.0 * (player_stats.level as f64).powf(1.5)) as i64;
                        player_stats.max_hp += 20;
                        player_stats.hp = player_stats.max_hp;
                        player_stats.attack += 2;
                        println!("🎉 레벨 업! Lv.{}", player_stats.level);
                    }
                    
                    println!("💀 {} 처치! +{} EXP, +{} Gold", monster_component.name, exp, gold);
                    
                    commands.entity(target_entity).despawn();
                }
            }
        }
    } else {
        combat_state.is_attacking = false;
    }
}

// ============ Cleanup ============

pub fn cleanup_game_world(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    players: Query<Entity, With<PlayerComponent>>,
    monsters: Query<Entity, With<MonsterComponent>>,
    hud: Query<Entity, With<super::components::HudUI>>,
) {
    for entity in tiles.iter().chain(players.iter()).chain(monsters.iter()).chain(hud.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}
