//! Legend of Darkness M - Shared Library
//!
//! This crate contains shared code between client and server.

pub mod shared;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;
