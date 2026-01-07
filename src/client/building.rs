use bevy::prelude::*;
use crate::client::game::GameResources;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_build_mode, update_ghost, place_building));
    }
}

#[derive(Component)]
pub struct Building;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Wall,
    Base,
}

#[derive(Resource)]
pub struct BuildMode {
    pub active: bool,
    pub building_type: BuildingType,
    pub ghost_entity: Option<Entity>,
}

impl Default for BuildMode {
    fn default() -> Self {
        Self {
            active: false,
            building_type: BuildingType::Wall,
            ghost_entity: None,
        }
    }
}

fn toggle_build_mode(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut build_mode: ResMut<BuildMode>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        build_mode.active = !build_mode.active;

        // Despawn ghost if disabling
        if !build_mode.active {
            if let Some(entity) = build_mode.ghost_entity {
                commands.entity(entity).despawn_recursive();
                build_mode.ghost_entity = None;
            }
        } else {
             // Spawn ghost if enabling
             let mesh = meshes.add(Cuboid::new(1.0, 2.0, 1.0));
             let material = materials.add(StandardMaterial {
                 base_color: Color::srgba(0.5, 0.5, 1.0, 0.5),
                 alpha_mode: AlphaMode::Blend,
                 ..default()
             });

             let ghost = commands.spawn((
                 Mesh3d(mesh),
                 MeshMaterial3d(material),
                 Transform::from_xyz(0.0, -10.0, 0.0), // Hide initially
             )).id();

             build_mode.ghost_entity = Some(ghost);
        }
    }
}

fn update_ghost(
    build_mode: Res<BuildMode>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::client::camera::MainCamera>>,
    mut ghost_q: Query<&mut Transform>,
) {
    if !build_mode.active {
        return;
    }

    if let Some(ghost_entity) = build_mode.ghost_entity {
        if let Ok(mut transform) = ghost_q.get_mut(ghost_entity) {
            let (camera, camera_transform) = camera_q.single();
            let window = windows.single();

            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    if ray.direction.y.abs() > f32::EPSILON {
                        let t = -ray.origin.y / ray.direction.y;
                        if t >= 0.0 {
                            let target_pos = ray.origin + ray.direction * t;
                            transform.translation = Vec3::new(target_pos.x, 1.0, target_pos.z);
                        }
                    }
                }
            }
        }
    }
}

fn place_building(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    build_mode: Res<BuildMode>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::client::camera::MainCamera>>,
    mut game_resources: ResMut<GameResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !build_mode.active || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Cost Check (Simple Wall cost: 20 Wood)
    if game_resources.wood < 20 {
        info!("Not enough wood!");
        return;
    }

    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
             if ray.direction.y.abs() > f32::EPSILON {
                let t = -ray.origin.y / ray.direction.y;
                if t >= 0.0 {
                    let target_pos = ray.origin + ray.direction * t;

                    // Deduct resources
                    game_resources.wood -= 20;

                    // Place actual building
                    let mesh = meshes.add(Cuboid::new(1.0, 2.0, 1.0));
                    let material = materials.add(Color::srgb(0.5, 0.5, 0.5));

                    commands.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::from_xyz(target_pos.x, 1.0, target_pos.z),
                        Building,
                    ));
                }
            }
        }
    }
}
