use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveManager {
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
            wave_count: 0,
        });
        app.add_systems(Update, (spawn_waves, enemy_chase_player));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct WaveManager {
    pub timer: Timer,
    pub wave_count: u32,
}

fn spawn_waves(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wave_manager.timer.tick(time.delta());

    if wave_manager.timer.finished() {
        wave_manager.wave_count += 1;
        info!("Wave {} Started!", wave_manager.wave_count);

        let enemy_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let enemy_mat = materials.add(Color::srgb(0.8, 0.1, 0.1));

        // Spawn enemies based on wave count
        let count = 2 + wave_manager.wave_count * 2;

        for i in 0..count {
            // Random position at edge
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let radius = 20.0;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            commands.spawn((
                Mesh3d(enemy_mesh.clone()),
                MeshMaterial3d(enemy_mat.clone()),
                Transform::from_xyz(x, 0.5, z),
                Enemy,
            ));
        }
    }
}

fn enemy_chase_player(
    // Targets: Player or Buildings
    target_q: Query<&Transform, Or<(With<crate::client::player::Player>, With<crate::client::building::Building>)>>,
    mut enemy_q: Query<&mut Transform, (With<Enemy>, Without<crate::client::player::Player>, Without<crate::client::building::Building>)>,
    time: Res<Time>,
) {
    // Optimization: Find nearest target for each enemy (simple O(N*M))
    // For small counts this is fine.

    for mut enemy_transform in enemy_q.iter_mut() {
        let mut nearest_target: Option<Vec3> = None;
        let mut min_dist_sq = f32::MAX;

        for target_transform in target_q.iter() {
            let dist_sq = target_transform.translation.distance_squared(enemy_transform.translation);
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_target = Some(target_transform.translation);
            }
        }

        if let Some(target_pos) = nearest_target {
            let direction = target_pos - enemy_transform.translation;
            let distance = direction.length();

            if distance > 1.2 {
                let move_speed = 2.0;
                let move_dist = move_speed * time.delta_secs();
                enemy_transform.translation += direction.normalize() * move_dist;
                enemy_transform.look_at(target_pos, Vec3::Y);
            }
        }
    }
}
