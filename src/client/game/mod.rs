//! 게임 모듈
//!
//! hydrate feature가 활성화된 경우에만 컴파일



use leptos::prelude::*;
use leptos::ev;

use crate::shared::domain::*;

mod canvas;
mod systems;

pub use canvas::*;
pub use systems::*;

#[component]
pub fn GameView() -> impl IntoView {
    // 게임 상태
    let (player, set_player) = signal(Player::new(
        "Hero".to_string(),
        PlayerClass::Warrior,
    ));
    let (monsters, set_monsters) = signal(Vec::<Monster>::new());
    let (show_character, set_show_character) = signal(false);
    let (show_inventory, set_show_inventory) = signal(false);
    let (show_skills, set_show_skills) = signal(false);
    
    // 키보드 입력
    let (keys_pressed, set_keys_pressed) = signal(std::collections::HashSet::<String>::new());
    
    // 게임 초기화
    Effect::new(move |_| {
        let initial_monsters = vec![
            Monster::new(
                &MonsterData {
                    id: 1,
                    name: "슬라임".to_string(),
                    level: 1,
                    max_hp: 30,
                    attack_min: 2,
                    attack_max: 4,
                    defense: 1,
                    exp_reward: 5,
                    gold_min: 2,
                    gold_max: 5,
                    ai_type: MonsterAIType::Passive,
                    detection_range: 150.0,
                    attack_range: 40.0,
                    move_speed: 80.0,
                    sprite_path: "/assets/monsters/slime/spritesheet.png".to_string(),
                },
                Position::new(300.0, 200.0),
            ),
            Monster::new(
                &MonsterData {
                    id: 2,
                    name: "파란 슬라임".to_string(),
                    level: 2,
                    max_hp: 45,
                    attack_min: 3,
                    attack_max: 6,
                    defense: 2,
                    exp_reward: 8,
                    gold_min: 3,
                    gold_max: 8,
                    ai_type: MonsterAIType::Passive,
                    detection_range: 180.0,
                    attack_range: 45.0,
                    move_speed: 90.0,
                    sprite_path: "/assets/monsters/slime/spritesheet.png".to_string(),
                },
                Position::new(500.0, 350.0),
            ),
            Monster::new(
                &MonsterData {
                    id: 3,
                    name: "고블린".to_string(),
                    level: 6,
                    max_hp: 110,
                    attack_min: 13,
                    attack_max: 20,
                    defense: 8,
                    exp_reward: 25,
                    gold_min: 15,
                    gold_max: 30,
                    ai_type: MonsterAIType::Aggressive,
                    detection_range: 250.0,
                    attack_range: 55.0,
                    move_speed: 110.0,
                    sprite_path: "/assets/monsters/goblin/spritesheet.png".to_string(),
                },
                Position::new(600.0, 250.0),
            ),
        ];
        set_monsters.set(initial_monsters);
        
        window_event_listener(ev::keydown, move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            set_keys_pressed.update(|keys| {
                keys.insert(key);
            });
        });
        
        window_event_listener(ev::keyup, move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            set_keys_pressed.update(|keys| {
                keys.remove(&key);
            });
        });
    });

    view! {
        <div class="game-container">
            <crate::client::components::HUD
                player=player
                on_character_click=move |_| set_show_character.set(!show_character.get())
                on_inventory_click=move |_| set_show_inventory.set(!show_inventory.get())
                on_skills_click=move |_| set_show_skills.set(!show_skills.get())
            />
            
            <GameCanvas
                player=player
                set_player=set_player
                monsters=monsters
                keys_pressed=keys_pressed
            />
            
            {move || {
                if show_character.get() {
                    Some(view! {
                        <crate::client::components::CharacterWindow
                            player=player
                            set_player=set_player
                            on_close=move |_| set_show_character.set(false)
                        />
                    })
                } else {
                    None
                }
            }}
            
            {move || {
                if show_inventory.get() {
                    Some(view! {
                        <crate::client::components::InventoryWindow on_close=move |_| set_show_inventory.set(false) />
                    })
                } else {
                    None
                }
            }}
            
            {move || {
                if show_skills.get() {
                    Some(view! {
                        <crate::client::components::SkillWindow player=player on_close=move |_| set_show_skills.set(false) />
                    })
                } else {
                    None
                }
            }}
            
            <div class="game-info">
                <p>"WASD - 이동 | C - 캐릭터 | I - 인벤토리 | K - 스킬"</p>
                <p>"현재 맵: " {move || player.get().current_map.clone()}</p>
            </div>
        </div>
    }
}
