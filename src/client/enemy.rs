use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_enemies);
        app.add_systems(Update, enemy_chase_player);
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_initial_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let enemy_mat = materials.add(Color::srgb(0.8, 0.1, 0.1));

    // Spawn a few enemies around
    let positions = [
        Vec3::new(5.0, 0.5, 5.0),
        Vec3::new(-5.0, 0.5, 5.0),
        Vec3::new(5.0, 0.5, -5.0),
        Vec3::new(-5.0, 0.5, -5.0),
    ];

    for pos in positions {
        commands.spawn((
            Mesh3d(enemy_mesh.clone()),
            MeshMaterial3d(enemy_mat.clone()),
            Transform::from_translation(pos),
            Enemy,
        ));
    }
}

fn enemy_chase_player(
    player_q: Query<&Transform, (With<crate::client::player::Player>, Without<Enemy>)>,
    mut enemy_q: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        for mut enemy_transform in enemy_q.iter_mut() {
            let direction = player_transform.translation - enemy_transform.translation;
            let distance = direction.length();

            // Simple chase logic
            if distance > 1.2 { // Don't overlap completely
                let move_speed = 2.0;
                let move_dist = move_speed * time.delta_secs();
                enemy_transform.translation += direction.normalize() * move_dist;
                enemy_transform.look_at(player_transform.translation, Vec3::Y);
            }
        }
    }
}
