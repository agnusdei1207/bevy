use leptos::prelude::*;
use crate::shared::domain::*;

#[component]
pub fn HUD(
    player: ReadSignal<Player>,
    on_character_click: impl Fn(web_sys::MouseEvent) + 'static,
    on_inventory_click: impl Fn(web_sys::MouseEvent) + 'static,
    on_skills_click: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div class="hud">
            <div class="hud-top">
                <div class="player-info">
                    <div class="player-avatar" style=move || {
                        let color = match player.get().class {
                            PlayerClass::Warrior => "#ff4444",
                            PlayerClass::Rogue => "#44ff44",
                            PlayerClass::Mage => "#4444ff",
                            PlayerClass::Cleric => "#ff44ff",
                            PlayerClass::MartialArtist => "#ffff44",
                        };
                        format!("background-color: {}", color)
                    }></div>
                    <div class="player-details">
                        <div class="player-name">{move || player.get().username.clone()}</div>
                        <div class="player-class">{move || player.get().class.name().to_string()}</div>
                        <div class="player-level">"Lv. " {move || player.get().level}</div>
                    </div>
                </div>
                
                <div class="status-bars">
                    <div class="status-bar">
                        <span class="stat-label">"HP"</span>
                        <div class="bar hp-bar">
                            <div class="bar-fill" style=move || {
                                let p = player.get();
                                let percent = (p.combat_stats.hp * 100) / p.combat_stats.max_hp;
                                format!("width: {}%", percent)
                            }></div>
                        </div>
                        <span class="stat-value">{move || {
                            let p = player.get();
                            format!("{}/{}", p.combat_stats.hp, p.combat_stats.max_hp)
                        }}</span>
                    </div>
                    <div class="status-bar">
                        <span class="stat-label">"MP"</span>
                        <div class="bar mp-bar">
                            <div class="bar-fill" style=move || {
                                let p = player.get();
                                let percent = (p.combat_stats.mp * 100) / p.combat_stats.max_mp;
                                format!("width: {}%", percent)
                            }></div>
                        </div>
                        <span class="stat-value">{move || {
                            let p = player.get();
                            format!("{}/{}", p.combat_stats.mp, p.combat_stats.max_mp)
                        }}</span>
                    </div>
                    <div class="exp-bar">
                        <span class="exp-label">"EXP"</span>
                        <div class="bar exp-bar-fill">
                            <div class="bar-fill" style=move || {
                                let p = player.get();
                                let percent = (p.exp * 100) / p.exp_to_next_level;
                                format!("width: {}%", percent)
                            }></div>
                        </div>
                        <span class="exp-value">{move || {
                            let p = player.get();
                            format!("{}/{}", p.exp, p.exp_to_next_level)
                        }}</span>
                    </div>
                </div>
                
                <div class="gold-display">
                    <span class="gold-icon">"💰"</span>
                    <span class="gold-amount">{move || player.get().gold}</span>
                </div>
            </div>
            
            <div class="hud-menu">
                <button class="menu-btn" on:click=on_character_click>"캐릭터 (C)"</button>
                <button class="menu-btn" on:click=on_inventory_click>"인벤토리 (I)"</button>
                <button class="menu-btn" on:click=on_skills_click>"스킬 (K)"</button>
            </div>
        </div>
    }
}

