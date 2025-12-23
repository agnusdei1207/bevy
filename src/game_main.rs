//! Legend of Darkness M - Bevy Game Client
//!
//! Run: cargo run --bin legend-game --features client

#[cfg(feature = "client")]
fn main() {
    use legend_client::client::LegendGamePlugin;
    use bevy::prelude::*;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "어둠의전설 M - Legend of Darkness".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LegendGamePlugin)
        .run();
}

#[cfg(not(feature = "client"))]
fn main() {
    println!("Run with: cargo run --bin legend-game --features client");
}
