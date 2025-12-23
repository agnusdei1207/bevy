//! App component - CSR Only

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::{Router, Route, Routes};

use crate::client::game;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="어둠의전설 M - Legend of Darkness M"/>
        <Meta name="description" content="픽셀 RPG 온라인 게임"/>
        
        <Router>
            <main>
                <Routes fallback=|| "Not Found">
                    <Route path=path!("/") view=GamePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn GamePage() -> impl IntoView {
    view! { <game::GameView/> }
}
