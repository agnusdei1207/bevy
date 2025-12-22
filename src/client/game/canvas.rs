use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use crate::shared::domain::*;
use std::collections::HashSet;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

#[component]
pub fn GameCanvas(
    player: ReadSignal<Player>,
    set_player: WriteSignal<Player>,
    monsters: ReadSignal<Vec<Monster>>,
    keys_pressed: ReadSignal<HashSet<String>>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let (tile_sheet, set_tile_sheet) = signal(None::<web_sys::HtmlImageElement>);
    let (building_sheet, set_building_sheet) = signal(None::<web_sys::HtmlImageElement>);
    
    // Map System
    let map_data = crate::client::game::systems::MapRenderer::create_mock_map();
    let map_renderer = crate::client::game::systems::MapRenderer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // Load Tile Sheet
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/iso_tiles.png");
        let set_img = set_tile_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // Load Building Sheet
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/iso_buildings.png");
        let set_img = set_building_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // Load Lamp Sheet (Yellow Torch)
    let (lamp_sheet, set_lamp_sheet) = signal(None::<web_sys::HtmlImageElement>);
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/lamp_yellow.png");
        let set_img = set_lamp_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // ... (existing effects) ...
    // Update render loop
    Effect::new(move |_| {
    // ...
                    render_game(
                        &ctx, 
                        player.get(), 
                        monsters.get(), 
                        &char_sheet.get(), 
                        &monster_sheet.get(), 
                        &tile_sheet.get(),
                        &building_sheet.get(),
                        &lamp_sheet.get(),
                        &map_renderer,
                        &map_data
                    );
    // ...
    });

    // Keeping the rest of the file...
    // Note: I will need to replace the render_game signature and implementation in one go properly.
    // I am replacing the TOP part of the component logic here.
    
    let (player_sprite, set_player_sprite) = signal(None::<web_sys::HtmlImageElement>);
    let (monster_sheet, set_monster_sheet) = signal(None::<web_sys::HtmlImageElement>); // Ensuring this exists too

    // Load Player Sprite Dynamically
    Effect::new(move |_| {
        let p = player.get();
        let class_name = match p.class {
            PlayerClass::Warrior => "warrior",
            PlayerClass::Rogue => "rogue",
            PlayerClass::Mage => "mage",
            PlayerClass::Cleric => "cleric",
            PlayerClass::MartialArtist => "martial_artist",
        };
        let suffix = if p.gender == "female" { "_female" } else { "" };
        let path = format!("/assets/characters/{}{}.png", class_name, suffix);
        
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src(&path);
        let set_img = set_player_sprite;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // ... (existing effects) ...

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
                    // Update logic
                    let keys = keys_pressed.get();
                    let mut p = player.get();
                    let speed = 200.0 * delta_time;
                    let mut new_x = p.position.x;
                    let mut new_y = p.position.y;
                    let mut moved = false;
                    
                    if keys.contains("ArrowUp") || keys.contains("w") || keys.contains("W") { new_y -= speed; p.direction = Direction::Up; moved = true; }
                    if keys.contains("ArrowDown") || keys.contains("s") || keys.contains("S") { new_y += speed; p.direction = Direction::Down; moved = true; }
                    if keys.contains("ArrowLeft") || keys.contains("a") || keys.contains("A") { new_x -= speed; p.direction = Direction::Left; moved = true; }
                    if keys.contains("ArrowRight") || keys.contains("d") || keys.contains("D") { new_x += speed; p.direction = Direction::Right; moved = true; }
                    
                    // Attack (Spacebar)
                    if keys.contains(" ") {
                        if p.can_attack(current_time) {
                            p.register_attack(current_time);
                        }
                    }

                    // Reset attacking state after a short animation window (e.g. 500ms)
                    if p.is_attacking && (current_time - p.last_attack_time > 500.0) {
                        p.is_attacking = false;
                    }
                    
                    new_x = new_x.max(16.0).min(CANVAS_WIDTH - 16.0);
                    new_y = new_y.max(16.0).min(CANVAS_HEIGHT - 16.0);
                    
                    p.position.x = new_x;
                    p.position.y = new_y;
                    p.is_moving = moved;
                    set_player.set(p);
                    
                    render_game(
                        &ctx, 
                        player.get(), 
                        monsters.get(), 
                        &player_sprite.get(), 
                        &monster_sheet.get(), 
                        &tile_sheet.get(),
                        &building_sheet.get(),
                        &lamp_sheet.get(),
                        &map_renderer,
                        &map_data
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

fn render_game(
    ctx: &CanvasRenderingContext2d, 
    player: Player, 
    monsters: Vec<Monster>, 
    player_sprite: &Option<web_sys::HtmlImageElement>,
    monster_sheet: &Option<web_sys::HtmlImageElement>,
    tile_sheet: &Option<web_sys::HtmlImageElement>,
    building_sheet: &Option<web_sys::HtmlImageElement>,
    lamp_sheet: &Option<web_sys::HtmlImageElement>,
    map_renderer: &crate::client::game::systems::MapRenderer,
    map_data: &crate::shared::domain::map::MapData
) {
    // 배경
    ctx.set_fill_style(&JsValue::from_str("#0f0c29"));
    ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // 맵 렌더링
    map_renderer.render(ctx, map_data, tile_sheet, building_sheet, lamp_sheet);
    
    // Sort entities
    enum Entity<'a> {
        Player(&'a Player),
        Monster(&'a Monster),
    }
    
    let mut entities: Vec<Entity> = Vec::new();
    entities.push(Entity::Player(&player));
    for m in &monsters {
        entities.push(Entity::Monster(m));
    }
    
    entities.sort_by(|a, b| {
        let (ax, ay) = match a {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        let (bx, by) = match b {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        (ax + ay).partial_cmp(&(bx + by)).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for entity in entities {
        match entity {
            Entity::Player(p) => {
                let (sx, sy) = to_screen_coord(p.position.x, p.position.y);
                if let Some(img) = player_sprite {
                    draw_player_sprite(ctx, p, img, sx, sy);
                } else {
                    draw_player(ctx, p, sx, sy);
                }
            },
            Entity::Monster(m) => {
                let (sx, sy) = to_screen_coord(m.position.x, m.position.y);
                if let Some(img) = monster_sheet {
                    draw_monster_sprite(ctx, m, img, sx, sy);
                } else {
                    draw_monster(ctx, m, sx, sy);
                }
            }
        }
    }
}

// ... helper functions ...

fn draw_player_sprite(ctx: &CanvasRenderingContext2d, player: &Player, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 32.0; // Centered 64 width
    let draw_y = sy - 48.0; // 64 height
    
    // Sprite Layout Assumption for "Class Specific Sheet":
    // Row 0: Idle
    // Row 1: Move
    // Row 2: Attack
    
    let mut row = 0.0;
    if player.is_attacking {
        row = 2.0;
    } else if player.is_moving {
        row = 1.0;
    } else {
        row = 0.0;
    }
    
    // Animation Frame Logic
    let now = js_sys::Date::now() as f64;
    let frame_count = 4; // Assumption
    let frame_duration = 150.0; // ms
    
    let frame = if player.is_attacking {
        let attack_elapsed = now - player.last_attack_time;
        // Attack animation over 500ms approx
        let total_frames = 6; // User requested 6 frames for skills/attack if possible
        // Map 0-500ms to 0-5
        let idx = (attack_elapsed / (500.0 / total_frames as f64)).floor();
        idx.min((total_frames - 1) as f64)
    } else {
        ((now / frame_duration) as i32 % frame_count) as f64
    };

    let cell_width = 64.0; 
    let cell_height = 64.0;
    
    let src_x = frame * cell_width;
    let src_y = row * cell_height;
    
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, src_x, src_y, cell_width, cell_height, draw_x, draw_y, cell_width, cell_height
    ).unwrap_or_else(|_| ());
    
    // Name
    ctx.set_fill_style(&JsValue::from_str("#c5c6c7"));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&player.username, sx, draw_y - 5.0).unwrap();

    // HP bar
    draw_hp_bar(ctx, sx, draw_y - 8.0, player.combat_stats.hp, player.combat_stats.max_hp);
}

fn draw_monster_sprite(ctx: &CanvasRenderingContext2d, monster: &Monster, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 16.0;
    let draw_y = sy - 24.0;
    
    // Map monster name to index in sprite sheet
    // Generated sheet has 8 monsters.
    // Order: Rat, Bat, Skeleton, Kobold, Spider, Ghoul, Werewolf, Succubus
    // Assuming single row or grid. The prompt asked for specific list.
    // Let's assume they are laid out in a grid or row.
    // 0: Rat, 1: Bat ...
    
    let index = match monster.name.as_str() {
        "쥐" => 0.0,
        "박쥐" => 1.0,
        "스켈레톤" => 2.0,
        "코볼트" => 3.0,
        "거대 거미" => 4.0,
        "구울" => 5.0,
        "라이칸스로프" => 6.0,
        "서큐버스" => 7.0,
        _ => 0.0, // Default
    };
    
    // Assuming standard 32x32 logic for monsters or 32x48? Monster output often square.
    // Let's assume 32x32 for most, maybe larger for later ones.
    // If they are all in one sheet side-by-side:
    let src_x = index * 48.0; // Spacing?
    // Actually generated image likely put them in 2 rows of 4 if typical.
    // Let's guess: 4 columns, 2 rows.
    
    let col = index % 4.0;
    let row = (index / 4.0).floor();
    
    // Assuming 64x64 sprites for clarity in spritesheet? Or 32x32?
    // Let's try 32x32 scale.
    let cell_size = 48.0; // monsters can be bigger
    
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, col * cell_size, row * cell_size, cell_size, cell_size, draw_x - 8.0, draw_y - 16.0, cell_size, cell_size
    ).unwrap_or_else(|_| ());
    
    ctx.set_fill_style(&JsValue::from_str("#ff4444"));
    ctx.set_font("8px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&monster.name, sx, draw_y - 5.0).unwrap();
    
    draw_hp_bar(ctx, sx, draw_y - 8.0, monster.hp, monster.max_hp);
}


// Keeping fallback functions for reference or unimplemented types
// Keeping fallback functions for reference or unimplemented types
fn draw_player(ctx: &CanvasRenderingContext2d, player: &Player, sx: f64, sy: f64) {
    ctx.set_fill_style(&JsValue::from_str("#ff0000"));
    ctx.fill_rect(sx - 8.0, sy - 8.0, 16.0, 16.0);

    // Name
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&player.username, sx, sy - 15.0).unwrap();
    
    // HP bar
    draw_hp_bar(ctx, sx, sy - 20.0, player.combat_stats.hp, player.combat_stats.max_hp);
}

fn draw_monster(ctx: &CanvasRenderingContext2d, monster: &Monster, sx: f64, sy: f64) {
    ctx.set_fill_style(&JsValue::from_str("#00ff00"));
    ctx.fill_rect(sx - 8.0, sy - 8.0, 16.0, 16.0);

    // Name
    ctx.set_fill_style(&JsValue::from_str("#ffff00"));
    ctx.set_font("8px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&monster.name, sx, sy - 12.0).unwrap();
    
    // HP bar
    draw_hp_bar(ctx, sx, sy - 18.0, monster.hp, monster.max_hp);
}

fn draw_hp_bar(ctx: &CanvasRenderingContext2d, x: f64, y: f64, hp: i32, max_hp: i32) {
    let bar_width = 30.0;
    let bar_height = 4.0;
    let hp_ratio = hp as f64 / max_hp as f64;
    
    // Background
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width, bar_height);
    
    // HP
    let hp_color = if hp_ratio > 0.5 {
        "#00ff00"
    } else if hp_ratio > 0.25 {
        "#ffff00"
    } else {
        "#ff0000"
    };
    
    ctx.set_fill_style(&JsValue::from_str(hp_color));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width * hp_ratio, bar_height);
    
    // Border
    ctx.set_stroke_style(&JsValue::from_str("#ffffff"));
    ctx.set_line_width(0.5);
    ctx.stroke_rect(x - bar_width / 2.0, y, bar_width, bar_height);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(closure: &Closure<dyn FnMut()>) -> i32;
}
