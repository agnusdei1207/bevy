pub mod building;
pub mod camera;
pub mod enemy;
pub mod game;
pub mod map;
pub mod player;

use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<building::BuildMode>();
        app.add_plugins((
            game::GamePlugin,
            camera::CameraPlugin,
            map::MapPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            building::BuildingPlugin,
        ));
    }
}
