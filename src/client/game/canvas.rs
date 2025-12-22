//! 게임 캔버스 - 스프라이트시트 기반 렌더링
//! 
//! 90년대 RPG 스타일의 아이소메트릭 2.5D 렌더링
//! 모든 엔티티는 1타일만 차지 (시각적 크기 무관)
//!
//! hydrate feature가 활성화된 경우에만 컴파일

#![cfg(feature = "hydrate")]

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use crate::shared::domain::*;
use crate::client::game::systems::{
    AnimationState, AnimationCalculator, SpriteSheetInfo, TileRenderer,
    character_path, monster_path, tileset_path, buildings_path, decorations_path
};
use std::collections::HashSet;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

// 다크 판타지 색상 팔레트
const COLOR_BACKGROUND: &str = "#0a0a0a";
const COLOR_TEXT_PLAYER: &str = "#c5c6c7";
const COLOR_TEXT_MONSTER: &str = "#8b0000";
const COLOR_HP_HIGH: &str = "#2d4a22";
const COLOR_HP_MED: &str = "#8b7355";
const COLOR_HP_LOW: &str = "#660000";

#[component]
pub fn GameCanvas(
    player: ReadSignal<Player>,
    set_player: WriteSignal<Player>,
    monsters: ReadSignal<Vec<Monster>>,
    keys_pressed: ReadSignal<HashSet<String>>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    
    // 스프라이트시트 로딩
    let (tileset, set_tileset) = signal(None::<web_sys::HtmlImageElement>);
    let (buildings, set_buildings) = signal(None::<web_sys::HtmlImageElement>);
    let (decorations, set_decorations) = signal(None::<web_sys::HtmlImageElement>);
    let (player_sheet, set_player_sheet) = signal(None::<web_sys::HtmlImageElement>);
    let (monster_sheets, set_monster_sheets) = signal(std::collections::HashMap::<String, web_sys::HtmlImageElement>::new());
    
    // Map System
    let map_data = crate::client::game::systems::MapRenderer::create_mock_map();
    let map_renderer = crate::client::game::systems::MapRenderer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // 타일셋 로딩
    Effect::new(move |_| {
        load_image(tileset_path(), set_tileset);
        load_image(buildings_path(), set_buildings);
        load_image(decorations_path("torch"), set_decorations);
    });
    
    // 플레이어 스프라이트시트 로딩
    Effect::new(move |_| {
        let p = player.get();
        let class_name = match p.class {
            PlayerClass::Warrior => "warrior",
            PlayerClass::Rogue => "rogue",
            PlayerClass::Mage => "mage",
            PlayerClass::Cleric => "cleric",
            PlayerClass::MartialArtist => "martial_artist",
        };
        let gender = if p.gender == "female" { "female" } else { "male" };
        let path = character_path(class_name, gender);
        load_image(path, set_player_sheet);
    });
    
    // 몬스터 스프라이트시트 로딩
    Effect::new(move |_| {
        let current_monsters = monsters.get();
        let mut sheets = monster_sheets.get();
        
        for monster in current_monsters.iter() {
            let monster_type = get_monster_type(&monster.name);
            if !sheets.contains_key(&monster_type) {
                let path = monster_path(&monster_type);
                if let Ok(img) = web_sys::HtmlImageElement::new() {
                    img.set_src(&path);
                    sheets.insert(monster_type, img);
                }
            }
        }
        set_monster_sheets.set(sheets);
    });

    // 게임 루프
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            
            ctx.set_image_smoothing_enabled(false);
            
            let f = std::rc::Rc::new(std::cell::RefCell::new(None::<Closure<dyn FnMut()>>));
            let g = f.clone();
            let mut last_time = js_sys::Date::now();
            
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let current_time = js_sys::Date::now();
                let delta_time = (current_time - last_time) / 1000.0;
                last_time = current_time;
                
                if delta_time > 0.0 && delta_time < 0.1 {
                    // 입력 처리
                    let keys = keys_pressed.get();
                    let mut p = player.get();
                    let speed = 200.0 * delta_time;
                    let mut new_x = p.position.x;
                    let mut new_y = p.position.y;
                    let mut moved = false;
                    
                    if keys.contains("ArrowUp") || keys.contains("w") || keys.contains("W") { 
                        new_y -= speed; p.direction = Direction::Up; moved = true; 
                    }
                    if keys.contains("ArrowDown") || keys.contains("s") || keys.contains("S") { 
                        new_y += speed; p.direction = Direction::Down; moved = true; 
                    }
                    if keys.contains("ArrowLeft") || keys.contains("a") || keys.contains("A") { 
                        new_x -= speed; p.direction = Direction::Left; moved = true; 
                    }
                    if keys.contains("ArrowRight") || keys.contains("d") || keys.contains("D") { 
                        new_x += speed; p.direction = Direction::Right; moved = true; 
                    }
                    
                    if keys.contains(" ") && p.can_attack(current_time) {
                        p.register_attack(current_time);
                    }

                    if p.is_attacking && AnimationCalculator::is_attack_finished(current_time, p.last_attack_time) {
                        p.is_attacking = false;
                    }
                    
                    new_x = new_x.max(32.0).min(CANVAS_WIDTH - 32.0);
                    new_y = new_y.max(32.0).min(CANVAS_HEIGHT - 32.0);
                    
                    p.position.x = new_x;
                    p.position.y = new_y;
                    p.is_moving = moved;
                    set_player.set(p);
                    
                    render_game(
                        &ctx, current_time, player.get(), monsters.get(),
                        &player_sheet.get(), &monster_sheets.get(),
                        &tileset.get(), &buildings.get(), &decorations.get(),
                        &map_renderer, &map_data
                    );
                }
                request_animation_frame(f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut()>));
            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    });

    view! {
        <canvas
            node_ref=canvas_ref
            width=CANVAS_WIDTH.to_string()
            height=CANVAS_HEIGHT.to_string()
            class="game-canvas"
        ></canvas>
    }
}

fn load_image(path: String, setter: WriteSignal<Option<web_sys::HtmlImageElement>>) {
    if let Ok(img) = web_sys::HtmlImageElement::new() {
        img.set_src(&path);
        let img_clone = img.clone();
        let onload = Closure::wrap(Box::new(move || {
            setter.set(Some(img_clone.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    }
}

fn get_monster_type(name: &str) -> String {
    match name {
        "슬라임" | "파란 슬라임" | "독 슬라임" => "slime",
        "쥐" | "거대 쥐" => "rat",
        "박쥐" | "흡혈 박쥐" => "bat",
        "늑대" | "야생 늑대" => "wolf",
        "스켈레톤" | "스켈레톤 전사" => "skeleton",
        "고블린" | "고블린 전사" => "goblin",
        "유령" | "원혼" => "ghost",
        "드래곤" | "고대 드래곤" => "dragon",
        _ => "slime",
    }.to_string()
}

fn render_game(
    ctx: &CanvasRenderingContext2d,
    current_time: f64,
    player: Player,
    monsters: Vec<Monster>,
    player_sheet: &Option<web_sys::HtmlImageElement>,
    monster_sheets: &std::collections::HashMap<String, web_sys::HtmlImageElement>,
    tileset: &Option<web_sys::HtmlImageElement>,
    buildings: &Option<web_sys::HtmlImageElement>,
    decorations: &Option<web_sys::HtmlImageElement>,
    map_renderer: &crate::client::game::systems::MapRenderer,
    map_data: &crate::shared::domain::map::MapData
) {
    // 배경
    ctx.set_fill_style(&JsValue::from_str(COLOR_BACKGROUND));
    ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // 맵 렌더링
    map_renderer.render(ctx, map_data, tileset, buildings, decorations);
    
    // 엔티티 정렬
    enum Entity<'a> { Player(&'a Player), Monster(&'a Monster) }
    
    let mut entities: Vec<Entity> = Vec::new();
    entities.push(Entity::Player(&player));
    for m in &monsters { entities.push(Entity::Monster(m)); }
    
    entities.sort_by(|a, b| {
        let (ax, ay) = match a { Entity::Player(p) => (p.position.x, p.position.y), Entity::Monster(m) => (m.position.x, m.position.y) };
        let (bx, by) = match b { Entity::Player(p) => (p.position.x, p.position.y), Entity::Monster(m) => (m.position.x, m.position.y) };
        (ax + ay).partial_cmp(&(bx + by)).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for entity in entities {
        match entity {
            Entity::Player(p) => draw_player_spritesheet(ctx, current_time, p, player_sheet),
            Entity::Monster(m) => {
                let monster_type = get_monster_type(&m.name);
                draw_monster_spritesheet(ctx, current_time, m, monster_sheets.get(&monster_type));
            }
        }
    }
}

fn draw_player_spritesheet(ctx: &CanvasRenderingContext2d, current_time: f64, player: &Player, sheet: &Option<web_sys::HtmlImageElement>) {
    let info = SpriteSheetInfo::character();
    
    let state = if player.is_attacking { AnimationState::Attack }
                else if player.is_moving { AnimationState::Move }
                else { AnimationState::Idle };
    
    let frame = if state == AnimationState::Attack {
        AnimationCalculator::get_oneshot_frame(current_time, player.last_attack_time, state)
    } else {
        AnimationCalculator::get_loop_frame(current_time, state)
    };
    
    let (src_x, src_y, src_w, src_h) = info.get_source_rect(state, frame);
    let (draw_x, draw_y) = TileRenderer::get_draw_position(player.position.x, player.position.y, info.frame_width, info.frame_height);
    
    if let Some(img) = sheet {
        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img, src_x, src_y, src_w, src_h, draw_x, draw_y, info.frame_width, info.frame_height
        );
    } else {
        draw_placeholder(ctx, player.position.x, player.position.y, "#660000", 16.0);
    }
    
    ctx.set_fill_style(&JsValue::from_str(COLOR_TEXT_PLAYER));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    let _ = ctx.fill_text(&player.username, player.position.x, draw_y - 5.0);
    draw_hp_bar(ctx, player.position.x, draw_y - 8.0, player.combat_stats.hp, player.combat_stats.max_hp);
}

fn draw_monster_spritesheet(ctx: &CanvasRenderingContext2d, current_time: f64, monster: &Monster, sheet: Option<&web_sys::HtmlImageElement>) {
    let info = if monster.level <= 10 { SpriteSheetInfo::small_monster() }
               else if monster.level <= 50 { SpriteSheetInfo::medium_monster() }
               else { SpriteSheetInfo::large_monster() };
    
    let state = if monster.is_dead() { AnimationState::Death }
                else if monster.is_attacking { AnimationState::Attack }
                else if monster.target_player_id.is_some() { AnimationState::Move }
                else { AnimationState::Idle };
    
    let frame = match state {
        AnimationState::Attack | AnimationState::Death => AnimationCalculator::get_oneshot_frame(current_time, monster.last_attack_time, state),
        _ => AnimationCalculator::get_loop_frame(current_time, state)
    };
    
    let (src_x, src_y, src_w, src_h) = info.get_source_rect(state, frame);
    let (draw_x, draw_y) = TileRenderer::get_draw_position(monster.position.x, monster.position.y, info.frame_width, info.frame_height);
    
    if let Some(img) = sheet {
        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img, src_x, src_y, src_w, src_h, draw_x, draw_y, info.frame_width, info.frame_height
        );
    } else {
        draw_placeholder(ctx, monster.position.x, monster.position.y, "#2d4a22", 12.0);
    }
    
    ctx.set_fill_style(&JsValue::from_str(COLOR_TEXT_MONSTER));
    ctx.set_font("8px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    let _ = ctx.fill_text(&monster.name, monster.position.x, draw_y - 5.0);
    draw_hp_bar(ctx, monster.position.x, draw_y - 8.0, monster.hp, monster.max_hp);
}

fn draw_placeholder(ctx: &CanvasRenderingContext2d, x: f64, y: f64, color: &str, size: f64) {
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(x - size / 2.0, y - size / 2.0, size, size);
    ctx.set_stroke_style(&JsValue::from_str("#1a1a2e"));
    ctx.set_line_width(1.0);
    ctx.stroke_rect(x - size / 2.0, y - size / 2.0, size, size);
}

fn draw_hp_bar(ctx: &CanvasRenderingContext2d, x: f64, y: f64, hp: i32, max_hp: i32) {
    let bar_width = 30.0;
    let bar_height = 4.0;
    let hp_ratio = hp as f64 / max_hp as f64;
    
    ctx.set_fill_style(&JsValue::from_str("#0a0a0a"));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width, bar_height);
    
    let hp_color = if hp_ratio > 0.5 { COLOR_HP_HIGH } else if hp_ratio > 0.25 { COLOR_HP_MED } else { COLOR_HP_LOW };
    ctx.set_fill_style(&JsValue::from_str(hp_color));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width * hp_ratio, bar_height);
    
    ctx.set_stroke_style(&JsValue::from_str("#1a1a2e"));
    ctx.set_line_width(0.5);
    ctx.stroke_rect(x - bar_width / 2.0, y, bar_width, bar_height);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(closure: &Closure<dyn FnMut()>) -> i32;
}
