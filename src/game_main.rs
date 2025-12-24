//! Legend of Darkness M - Bevy Game Client
//!
//! Run: cargo run --bin legend-game --features client

#[cfg(feature = "client")]
fn main() {
    use legend_client::client::LegendGamePlugin;
    use bevy::prelude::*;
    use bevy::asset::AssetMetaCheck;
    
    // WASM panic hook for better debugging
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "어둠의전설 M - Legend of Darkness".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: true,
                    canvas: Some("#bevy-canvas".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            })
            .set(bevy::asset::AssetPlugin {
                // Don't check for .meta files (they don't exist)
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
        )
        .add_plugins(LegendGamePlugin)
        .run();
}

#[cfg(not(feature = "client"))]
fn main() {
    println!("Run with: cargo run --bin legend-game --features client");
}
