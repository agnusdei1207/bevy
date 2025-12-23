//! Game States

use bevy::prelude::*;

/// Main game state machine
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    CharacterSelect,
    Playing,
    Paused,
    GameOver,
}
