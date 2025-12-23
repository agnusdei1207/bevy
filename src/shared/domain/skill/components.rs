//! Skill Window Component - CSR Version

use leptos::prelude::*;
use crate::shared::domain::skill::models::Skill;
use crate::shared::domain::skill::server::get_skills;

#[component]
pub fn SkillWindow(
    player_class: Signal<String>,
    player_level: Signal<i32>,
    on_close: impl Fn(web_sys::MouseEvent) + 'static + Clone,
) -> impl IntoView {
    // Resource to fetch skills (CSR version - uses mock data or HTTP)
    let skills_resource: Resource<Result<Vec<Skill>, String>> = Resource::new(
        move || (player_class.get(), player_level.get()),
        move |(class, level)| async move {
            get_skills(class, level).await
        }
    );

    view! {
        <div class="modal-overlay">
            <div class="window skill-window">
                <div class="window-header">
                    <h2>"스킬"</h2>
                    <button class="close-btn" on:click=on_close.clone()>"✕"</button>
                </div>
                
                <div class="window-content">
                    <div class="skill-info">
                        <p>"직업: " {move || player_class.get()}</p>
                        <p>"레벨 " {move || player_level.get()} " 스킬 목록"</p>
                    </div>
                    
                    <div class="skill-grid-container">
                        <Suspense fallback=move || view! { <div class="loading">"스킬 불러오는 중..."</div> }>
                            {move || {
                                skills_resource.get().map(|result: Result<Vec<Skill>, _>| match result {
                                    Ok(skills) => {
                                        if skills.is_empty() {
                                            view! { <div class="empty-message">"배울 수 있는 스킬이 없습니다."</div> }.into_any()
                                        } else {
                                            view! {
                                                <div class="skill-grid">
                                                    {skills.into_iter().map(|skill| {
                                                        let icon_path = skill.icon_path.clone();
                                                        let skill_name = skill.name.clone();
                                                        view! {
                                                            <div class="skill-slot" title={skill.description.unwrap_or_default()}>
                                                                <div class="skill-icon-wrapper">
                                                                   {if let Some(path) = icon_path {
                                                                       view! { <img src=path alt=skill_name class="skill-icon-img" /> }.into_any()
                                                                   } else {
                                                                       view! { <div class="skill-icon-placeholder">"⚔️"</div> }.into_any()
                                                                   }}
                                                                </div>
                                                                <div class="skill-details">
                                                                    <div class="skill-name">{skill.name}</div>
                                                                    <div class="skill-meta">
                                                                        <span class="skill-mp">"MP " {skill.mp_cost}</span>
                                                                        <span class="skill-dmg">"Dmg " {skill.damage_min}"~"{skill.damage_max}</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        }
                                    },
                                    Err(e) => view! { <div class="error-message">"오류 발생: " {e}</div> }.into_any()
                                })
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
