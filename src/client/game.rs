use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, _app: &mut App) {
        // Core game logic can go here (e.g., game state, score, waves)
        // For now, it just serves as a container for shared game resources if needed.
    }
}
