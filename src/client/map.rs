use bevy::prelude::*;
use rand::Rng;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Wood,
    Gold,
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Trees
    let tree_mesh = meshes.add(Cylinder::new(0.5, 2.0));
    let tree_mat = materials.add(Color::srgb(0.1, 0.4, 0.1));

    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x: f32 = rng.gen_range(-20.0..20.0);
        let z: f32 = rng.gen_range(-20.0..20.0);

        // Keep center clear for base
        if x.abs() < 5.0 && z.abs() < 5.0 {
            continue;
        }

        commands.spawn((
            Mesh3d(tree_mesh.clone()),
            MeshMaterial3d(tree_mat.clone()),
            Transform::from_xyz(x, 1.0, z),
            ResourceNode {
                resource_type: ResourceType::Wood,
                amount: 100,
            },
        ));
    }
}
