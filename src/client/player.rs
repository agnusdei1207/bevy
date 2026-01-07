use bevy::prelude::*;
use crate::client::game::GameResources;
use crate::client::map::{ResourceNode, ResourceType};

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
) {
    // Player Capsule
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 0.8))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player,
        PlayerState::Idle,
        Speed(6.0),
        GatherTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    ));
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
    mut player_q: Query<(Entity, &mut Transform, &Speed, &MovementTarget, &mut PlayerState), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((entity, mut transform, speed, target, mut state)) = player_q.get_single_mut() {
        let direction = target.0 - transform.translation;
        // Ignore Y for movement distance
        let flat_direction = Vec3::new(direction.x, 0.0, direction.z);
        let distance = flat_direction.length();

        if distance < 0.1 {
            // Arrived
            commands.entity(entity).remove::<MovementTarget>();

            // If we were moving, become idle. If gathering, stay gathering (logic handled in gather_resources)
            if let PlayerState::Moving = *state {
                *state = PlayerState::Idle;
            }
        } else {
            let move_dist = speed.0 * time.delta_secs();
            if move_dist >= distance {
                transform.translation = target.0;
                commands.entity(entity).remove::<MovementTarget>();
                 if let PlayerState::Moving = *state {
                    *state = PlayerState::Idle;
                }
            } else {
                transform.translation += flat_direction.normalize() * move_dist;
                // Rotate to face movement direction
                let look_target = Vec3::new(target.0.x, transform.translation.y, target.0.z);
                transform.look_at(look_target, Vec3::Y);
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
