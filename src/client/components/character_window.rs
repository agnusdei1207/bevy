use leptos::prelude::*;
use crate::shared::domain::*;

#[component]
pub fn CharacterWindow(
    player: ReadSignal<Player>,
    set_player: WriteSignal<Player>,
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    let add_str = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Str, 1));
    };
    let add_dex = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Dex, 1));
    };
    let add_int = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Int, 1));
    };
    let add_con = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Con, 1));
    };
    let add_wis = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Wis, 1));
    };
    
    view! {
        <div class="modal-overlay">
            <div class="window character-window">
                <div class="window-header">
                    <h2>"캐릭터 정보"</h2>
                    <button class="close-btn" on:click=on_close>"✕"</button>
                </div>
                
                <div class="window-content">
                    <div class="character-stats">
                        <h3>"기본 스탯"</h3>
                        <div class="stat-list">
                            <div class="stat-item">
                                <span>"힘 (STR)"</span>
                                <span>{move || player.get().stats.str}</span>
                                <button on:click=add_str>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"민첩 (DEX)"</span>
                                <span>{move || player.get().stats.dex}</span>
                                <button on:click=add_dex>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"지능 (INT)"</span>
                                <span>{move || player.get().stats.int}</span>
                                <button on:click=add_int>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"체력 (CON)"</span>
                                <span>{move || player.get().stats.con}</span>
                                <button on:click=add_con>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"지혜 (WIS)"</span>
                                <span>{move || player.get().stats.wis}</span>
                                <button on:click=add_wis>"+"</button>
                            </div>
                        </div>
                        <div class="stat-points">
                            "스탯 포인트: " {move || player.get().stat_points}
                        </div>
                    </div>
                    
                    <div class="combat-stats">
                        <h3>"전투 스탯"</h3>
                        <div class="stat-list">
                            <div class="stat-item">
                                <span>"공격력"</span>
                                <span>{move || {
                                    let p = player.get();
                                    format!("{}-{}", p.combat_stats.attack_min, p.combat_stats.attack_max)
                                }}</span>
                            </div>
                            <div class="stat-item">
                                <span>"방어력"</span>
                                <span>{move || player.get().combat_stats.defense}</span>
                            </div>
                            <div class="stat-item">
                                <span>"명중률"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.hit_rate)}</span>
                            </div>
                            <div class="stat-item">
                                <span>"회피율"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.avoid_rate)}</span>
                            </div>
                            <div class="stat-item">
                                <span>"크리티컬"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.critical_rate)}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

