use bevy::prelude::*;
use crate::client::game::GameResources;
use crate::client::map::{ResourceNode, ResourceType};
use crate::client::graphics::{Animation, create_sprite_material, create_sprite_mesh};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_input, move_player, gather_resources));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementTarget(Vec3);

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub enum PlayerState {
    Idle,
    Moving,
    Gathering(Entity), // Target Resource Entity
}

#[derive(Component)]
pub struct GatherTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load Player Sprite
    // Path based on ASSETS.md and file check
    let texture_handle = asset_server.load("characters/warrior/male/spritesheet.png");

    // Create Sprite Mesh (Billboard)
    // Size: 256x256 texture, 4x4 grid -> 64x64 frame.
    // In world units, let's say 1 unit = 1 meter. 64px could be 2.0 units height?
    // Let's approximate. Standard character height ~1.8m.
    let mesh_handle = create_sprite_mesh(&mut meshes, Vec2::new(2.0, 2.0));
    let material_handle = create_sprite_material(&mut materials, texture_handle, AlphaMode::Blend);

    // Player Entity (Container)
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        Player,
        PlayerState::Idle,
        Speed(6.0),
        GatherTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    ))
    .with_children(|parent| {
        // Sprite Entity
        parent.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
            Transform::from_xyz(0.0, 1.0, 0.0) // Lift up so feet are at (0,0,0) parent
                .with_rotation(Quat::from_rotation_x(-45.0f32.to_radians())), // Tilt back to face camera (approx)
                // Note: If using Billboard behavior, we'd use LookAt, but for fixed Iso, a fixed tilt is often used.
                // Or we can rotate the camera, and keep sprite vertical (billboard Y).
                // Let's try fixed vertical billboard first (always facing camera direction projected on ground).
                // Actually, for Diablo style, sprites are usually pre-rendered at 45 deg.
                // If we put them on a vertical quad, and camera is at 45 deg, it looks foreshortened.
                // To counteract foreshortening, we can tilt the quad back 45 degrees so it's perpendicular to camera.
                // Let's try 45 deg tilt.
            Animation::new(4, 4, 8.0), // 4 rows, 4 cols, 8 FPS
        ));
    });
}

fn player_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::client::camera::MainCamera>>,
    mut player_q: Query<(Entity, &mut PlayerState), With<Player>>,
    resource_q: Query<(Entity, &GlobalTransform), With<ResourceNode>>,
    mut commands: Commands,
) {
    // Right click to move or interact
    if mouse.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Raycast to ground
                if ray.direction.y.abs() > f32::EPSILON {
                    let t = -ray.origin.y / ray.direction.y;
                    if t >= 0.0 {
                        let target_pos = ray.origin + ray.direction * t;

                        // Check if we clicked a resource node (simple distance check for now)
                        // In a real game, use raycasting against mesh colliders
                        let mut clicked_resource = None;
                        for (res_entity, res_transform) in resource_q.iter() {
                            // Project resource to Y=0 plane for distance check
                            let res_pos_flat = Vec3::new(res_transform.translation().x, 0.0, res_transform.translation().z);
                            let click_pos_flat = Vec3::new(target_pos.x, 0.0, target_pos.z);

                            if res_pos_flat.distance(click_pos_flat) < 1.0 { // Radius of click
                                clicked_resource = Some((res_entity, res_pos_flat));
                                break;
                            }
                        }

                        if let Ok((player_entity, mut state)) = player_q.get_single_mut() {
                            if let Some((res_entity, res_pos)) = clicked_resource {
                                // Go to resource
                                commands.entity(player_entity).insert(MovementTarget(res_pos));
                                *state = PlayerState::Gathering(res_entity);
                            } else {
                                // Just move
                                commands.entity(player_entity).insert(MovementTarget(target_pos));
                                *state = PlayerState::Moving;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn move_player(
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Transform, &Speed, &MovementTarget, &mut PlayerState, &Children), With<Player>>,
    mut animation_q: Query<&mut Animation>,
    time: Res<Time>,
) {
    if let Ok((entity, mut transform, speed, target, mut state, children)) = player_q.get_single_mut() {
        let direction = target.0 - transform.translation;
        // Ignore Y for movement distance
        let flat_direction = Vec3::new(direction.x, 0.0, direction.z);
        let distance = flat_direction.length();

        let mut current_direction_idx = 0; // Down
        // Calculate direction index for spritesheet
        // Grid: Col 0 (Down), Col 1 (Left), Col 2 (Right), Col 3 (Up)
        if distance > 0.1 {
             let normalized = flat_direction.normalize();
             if normalized.z.abs() > normalized.x.abs() {
                 if normalized.z > 0.0 { current_direction_idx = 0; } // Down (+Z)
                 else { current_direction_idx = 3; } // Up (-Z)
             } else {
                 if normalized.x < 0.0 { current_direction_idx = 1; } // Left (-X)
                 else { current_direction_idx = 2; } // Right (+X)
             }
        }

        let mut is_moving = false;

        if distance < 0.1 {
            // Arrived
            commands.entity(entity).remove::<MovementTarget>();

            // If we were moving, become idle. If gathering, stay gathering (logic handled in gather_resources)
            if let PlayerState::Moving = *state {
                *state = PlayerState::Idle;
            }
        } else {
            is_moving = true;
            let move_dist = speed.0 * time.delta_secs();
            if move_dist >= distance {
                transform.translation = target.0;
                commands.entity(entity).remove::<MovementTarget>();
                 if let PlayerState::Moving = *state {
                    *state = PlayerState::Idle;
                }
            } else {
                transform.translation += flat_direction.normalize() * move_dist;
                // Don't rotate the container transform with LookAt, because that spins the billboard too.
                // We keep the container rotation fixed (or Identity) and just change the sprite frame.
                // transform.look_at(look_target, Vec3::Y); <--- REMOVED
            }
        }

        // Update Animation
        for child in children.iter() {
            if let Ok(mut animation) = animation_q.get_mut(*child) {
                 // Update Row (State)
                 // Grid: Row 0 (Idle), Row 1 (Walk), Row 2 (Attack), Row 3 (Die)
                 if is_moving {
                     animation.current_row = 1; // Walk
                 } else {
                     // Check if gathering (Attack animation?)
                     match *state {
                         PlayerState::Gathering(_) => animation.current_row = 2, // Attack/Gather
                         _ => animation.current_row = 0, // Idle
                     }
                 }

                 // Update Col (Direction) - BUT wait, the grid is Col=Direction?
                 // ASSETS.md: "Col 0 (Down) Col 1 (Left) Col 2 (Right) Col 3 (Up)"
                 // AND "Row 0 (Idle) Row 1 (Walk)..."
                 // This implies that for a given State (Row), there are 4 Direction frames (Cols).
                 // Wait, usually it's "Row per Direction" or "Row per Animation".
                 // If Row 0 is ALL Idle frames, then Col must represent Direction?
                 // If so, there is NO animation frames (movement of legs) within a direction?
                 // THAT IS WEIRD for a "spritesheet". Usually a spritesheet has multiple frames for walking.
                 //
                 // Let's re-read ASSETS.md carefully:
                 // "그리드: 4열 × 4행 = 16 프레임"
                 // "Col 0 (Down) Col 1 (Left) Col 2 (Right) Col 3 (Up)"
                 // "Row 0 (Idle) Row 1 (Walk) Row 2 (Attack) Row 3 (Die)"
                 //
                 // Interpretation A: It's a static image per direction/state. No walking animation cycle?
                 // Interpretation B: The documentation is simplifying. Maybe it's "Row = Direction", "Col = Frame"?
                 //
                 // Let's assume Interpretation A for now (Static sprite per state/dir) because 4x4 is very small.
                 // If it were animated, it would need more columns (e.g., 4 columns for Down-Walk).
                 // IF Interpretation A is true, then `current_col` should be `current_direction_idx`.
                 // And we should NOT auto-increment `current_col` in `graphics.rs`.

                 animation.current_col = current_direction_idx;

                 // Disable auto-play in graphics.rs or handle it here?
                 // The `animate_sprites` system in graphics.rs increments col.
                 // I should set `is_playing = false` so it doesn't loop through directions as if they were frames.
                 animation.is_playing = false;
            }
        }
    }
}

fn gather_resources(
    mut player_q: Query<(&mut PlayerState, &Transform, &mut GatherTimer), With<Player>>,
    mut resource_q: Query<(&mut ResourceNode, &GlobalTransform)>,
    mut game_resources: ResMut<GameResources>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if let Ok((mut state, player_transform, mut timer)) = player_q.get_single_mut() {
        if let PlayerState::Gathering(res_entity) = *state {
            if let Ok((mut node, res_transform)) = resource_q.get_mut(res_entity) {
                 let distance = player_transform.translation.distance(res_transform.translation());

                 // Gathering Range
                 if distance < 2.5 {
                     timer.0.tick(time.delta());
                     if timer.0.finished() {
                         // Add resources
                         match node.resource_type {
                             ResourceType::Wood => game_resources.wood += 10,
                             ResourceType::Gold => game_resources.gold += 10,
                         }

                         // Deplete node
                         if node.amount > 10 {
                             node.amount -= 10;
                         } else {
                             // Despawn
                             commands.entity(res_entity).despawn_recursive();
                             *state = PlayerState::Idle;
                         }
                     }
                 }
            } else {
                // Resource doesn't exist anymore
                *state = PlayerState::Idle;
            }
        }
    }
}
