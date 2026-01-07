use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_input, move_player));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementTarget(Vec3);

#[derive(Component)]
pub struct Speed(f32);

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
        Speed(6.0),
    ));
}

fn player_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::client::camera::MainCamera>>,
    player_q: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    // Right click to move (Diablo style)
    if mouse.pressed(MouseButton::Right) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Raycast to ground plane (y = 0)
                // ray.origin + t * ray.direction = (x, 0, z)
                // ray.origin.y + t * ray.direction.y = 0
                // t = -ray.origin.y / ray.direction.y

                if ray.direction.y.abs() > f32::EPSILON {
                    let t = -ray.origin.y / ray.direction.y;
                    if t >= 0.0 {
                        let target_pos = ray.origin + ray.direction * t;

                        if let Ok(entity) = player_q.get_single() {
                            commands.entity(entity).insert(MovementTarget(target_pos));
                        }
                    }
                }
            }
        }
    }
}

fn move_player(
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Transform, &Speed, &MovementTarget), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((entity, mut transform, speed, target)) = player_q.get_single_mut() {
        let direction = target.0 - transform.translation;
        let distance = direction.length();

        if distance < 0.1 {
            // Arrived
            commands.entity(entity).remove::<MovementTarget>();
        } else {
            let move_dist = speed.0 * time.delta_secs();
            if move_dist >= distance {
                transform.translation = target.0;
                commands.entity(entity).remove::<MovementTarget>();
            } else {
                transform.translation += direction.normalize() * move_dist;
                // Rotate to face movement direction
                let look_target = Vec3::new(target.0.x, transform.translation.y, target.0.z);
                transform.look_at(look_target, Vec3::Y);
            }
        }
    }
}
