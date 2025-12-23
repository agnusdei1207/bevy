//! Legend of Darkness M - CSR Entry Point

pub mod app;
pub mod client;
pub mod shared;

#[cfg(feature = "server")]
pub mod server;

// Re-export App
pub use app::App;

// CSR: WASM entry point
#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    use leptos::prelude::*;
    
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    
    log::info!("🎮 Legend of Darkness M - Starting...");
    
    mount_to_body(App);
}
