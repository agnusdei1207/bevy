use bevy::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, camera_follow_player);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    // Diablo-style Top-down 3D View
    // Steep angle (60 degrees) for better visibility of the battlefield
    let translation = Vec3::new(0.0, 15.0, 10.0);
    let camera_transform = Transform::from_translation(translation)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 45.0_f32.to_radians(), // Narrower FOV for less distortion
            ..default()
        }),
        camera_transform,
        MainCamera,
    ));
}

fn camera_follow_player(
    player_query: Query<&Transform, (With<crate::client::player::Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Fixed offset for Diablo-like static angle
            let offset = Vec3::new(0.0, 15.0, 10.0);
            let target_position = player_transform.translation;

            // Smooth follow could be added here, but direct assignment is snappy
            camera_transform.translation = target_position + offset;
            camera_transform.look_at(target_position, Vec3::Y);
        }
    }
}
