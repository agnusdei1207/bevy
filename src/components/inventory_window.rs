use leptos::prelude::*;

#[component]
pub fn InventoryWindow(
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div class="modal-overlay">
            <div class="window inventory-window">
                <div class="window-header">
                    <h2>"인벤토리"</h2>
                    <button class="close-btn" on:click=on_close>"✕"</button>
                </div>
                
                <div class="window-content">
                    <div class="inventory-grid">
                        {(0..16).map(|i| view! {
                            <div class="inventory-slot" data-slot=i>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    <div class="inventory-info">
                        <p>"24칸의 인벤토리"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
