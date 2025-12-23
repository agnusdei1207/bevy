//! Static game data definitions
//!
//! This module contains all game data constants that were previously stored in the database.
//! These are hardcoded for fast access and compile-time guarantees.

pub mod assets;
pub mod monsters;
pub mod characters;
pub mod skills;
pub mod items;
pub mod maps;

// Re-export commonly used types
pub use assets::*;
pub use monsters::*;
pub use characters::*;
pub use skills::*;
pub use items::*;
pub use maps::*;
