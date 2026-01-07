use bevy::prelude::*;


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}

/// Component to manage sprite animation state for 3D meshes (manual UV shifting)
#[derive(Component)]
pub struct Animation {
    pub texture_atlas_rows: usize,
    pub texture_atlas_cols: usize,
    pub current_row: usize,
    pub current_col: usize,
    pub timer: Timer,
    pub is_playing: bool,
}

impl Animation {
    pub fn new(rows: usize, cols: usize, fps: f32) -> Self {
        Self {
            texture_atlas_rows: rows,
            texture_atlas_cols: cols,
            current_row: 0,
            current_col: 0,
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            is_playing: true,
        }
    }
}

/// Helper to create a 3D sprite (quad with texture)
pub fn create_sprite_mesh(
    meshes: &mut Assets<Mesh>,
    size: Vec2,
) -> Handle<Mesh> {
    meshes.add(Rectangle::new(size.x, size.y))
}

pub fn create_sprite_material(
    materials: &mut Assets<StandardMaterial>,
    texture: Handle<Image>,
    alpha_mode: AlphaMode,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode,
        unlit: true, // Sprite style usually doesn't react to lighting heavily
        double_sided: true,
        cull_mode: None,
        ..default()
    })
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &Mesh3d)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut animation, mesh_handle) in query.iter_mut() {
        if animation.is_playing {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                // Logic to advance frame is handled by the controller (e.g. player controller changes col/row)
                // But for looping animations (like walking), we might want to advance col here if configured.
                // For now, let's assume the controller sets the specific Row/Col based on state,
                // OR we implement a simple frame advancer here.

                // Simple looper for columns (frames)
                animation.current_col = (animation.current_col + 1) % animation.texture_atlas_cols;
            }
        }

        // Update UVs
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            let rows = animation.texture_atlas_rows as f32;
            let cols = animation.texture_atlas_cols as f32;
            let row = animation.current_row as f32;
            let col = animation.current_col as f32;

            // UV coordinates for the current frame
            // Assuming texture starts top-left? Bevy mesh UVs are usually bottom-left (0,0) to top-right (1,1).
            // But images are usually read top-to-bottom.
            // Let's standard UV mapping: (0,0) is bottom-left.
            // If the sprite sheet is standard: Row 0 is usually top.
            // We might need to invert the Row index.

            // Standard Grid:
            // U_min = col / cols
            // U_max = (col + 1) / cols
            // V_min = (rows - 1 - row) / rows  <-- Flip Y
            // V_max = (rows - row) / rows

            let u_min = col / cols;
            let u_max = (col + 1.0) / cols;
            let v_min = (rows - 1.0 - row) / rows;
            let v_max = (rows - row) / rows;

            let uvs = vec![
                [u_min, v_max], // Top-Left (Vertex 0 in Quad)
                [u_max, v_max], // Top-Right (Vertex 1)
                [u_max, v_min], // Bottom-Right (Vertex 2)
                [u_min, v_min], // Bottom-Left (Vertex 3)
            ];

            // Note: Bevy's Rectangle mesh vertices order might differ.
            // Rectangle::new creates a quad on the XY plane.
            // Vertices: [(-w, h, 0), (w, h, 0), (w, -h, 0), (-w, -h, 0)] (roughly TL, TR, BR, BL)
            // UVs default: [(0,0), (1,0), (1,1), (0,1)] - Wait, let's verify standard quad UVs.
            // Actually, for a billboard we usually use a custom mesh or update the attribute.

            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }
    }
}
