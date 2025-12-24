//! Sprite System Module
//!
//! Legend of Darkness style sprite management system.
//! Supports:
//! - Manifest-based sprite configuration
//! - Paper doll (layering) system
//! - Directional animation with mirroring
//! - Palette swapping
//! - Anchor point system for equipment attachment

pub mod manifest;
pub mod layer;
pub mod anchor;

pub use manifest::*;
pub use layer::*;
pub use anchor::*;
