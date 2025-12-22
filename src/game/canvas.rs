use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use crate::models::*;
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
    let (spritesheet, set_spritesheet) = signal(None::<web_sys::HtmlImageElement>);
    
    // Load spritesheet
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/spritesheet.png");
        let set_img = set_spritesheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
            web_sys::console::log_1(&"Spritesheet loaded!".into());
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
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
            
            // 픽셀 렌더링 최적화
            ctx.set_image_smoothing_enabled(false);
            
            // 게임 루프
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
                        new_y -= speed;
                        p.direction = Direction::Up;
                        moved = true;
                    }
                    if keys.contains("ArrowDown") || keys.contains("s") || keys.contains("S") {
                        new_y += speed;
                        p.direction = Direction::Down;
                        moved = true;
                    }
                    if keys.contains("ArrowLeft") || keys.contains("a") || keys.contains("A") {
                        new_x -= speed;
                        p.direction = Direction::Left;
                        moved = true;
                    }
                    if keys.contains("ArrowRight") || keys.contains("d") || keys.contains("D") {
                        new_x += speed;
                        p.direction = Direction::Right;
                        moved = true;
                    }
                    
                    // 경계 체크
                    new_x = new_x.max(16.0).min(CANVAS_WIDTH - 16.0);
                    new_y = new_y.max(16.0).min(CANVAS_HEIGHT - 16.0);
                    
                    p.position.x = new_x;
                    p.position.y = new_y;
                    p.is_moving = moved;
                    
                    set_player.set(p);
                    
                    // 렌더링
                    render_game(&ctx, player.get(), monsters.get(), &spritesheet.get());
                }
                
                // 다음 프레임
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

// Isometric transformation
fn to_screen_coord(world_x: f64, world_y: f64) -> (f64, f64) {
    let tile_width = 64.0; // Visual width of a tile
    let tile_height = 32.0; // Visual height of a tile
    
    // Convert world pixels to grid coordinates roughly
    // Assuming world_x/y are still handled as 'pixels' in logic (e.g. 32px per tile logic)
    // We map 32 logical pixels -> 1 Isometric Tile
    
    let grid_x = world_x / 32.0;
    let grid_y = world_y / 32.0;
    
    let origin_x = CANVAS_WIDTH / 2.0;
    let origin_y = 100.0; // Start a bit down
    
    let screen_x = origin_x + (grid_x - grid_y) * (tile_width / 2.0);
    let screen_y = origin_y + (grid_x + grid_y) * (tile_height / 2.0);
    
    (screen_x, screen_y)
}

fn render_game(ctx: &CanvasRenderingContext2d, player: Player, monsters: Vec<Monster>, spritesheet: &Option<web_sys::HtmlImageElement>) {
    // 배경
    ctx.set_fill_style(&JsValue::from_str("#1a1a2e"));
    ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // 그리드
    draw_isometric_grid(ctx);
    
    // Entity sorting (Painter's Algorithm)
    // Create a list of renderable entities
    enum Entity<'a> {
        Player(&'a Player),
        Monster(&'a Monster),
    }
    
    let mut entities: Vec<Entity> = Vec::new();
    entities.push(Entity::Player(&player));
    for m in &monsters {
        entities.push(Entity::Monster(m));
    }
    
    // Sort by depth (screen Y roughly, or grid X+Y)
    entities.sort_by(|a, b| {
        let (ax, ay) = match a {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        let (bx, by) = match b {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        
        // Depth = x + y in standard isometric
        (ax + ay).partial_cmp(&(bx + by)).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for entity in entities {
        match entity {
            Entity::Player(p) => {
                let (sx, sy) = to_screen_coord(p.position.x, p.position.y);
                if let Some(img) = spritesheet {
                    draw_player_sprite(ctx, p, img, sx, sy);
                } else {
                    draw_player(ctx, p, sx, sy);
                }
            },
            Entity::Monster(m) => {
                let (sx, sy) = to_screen_coord(m.position.x, m.position.y);
                if let Some(img) = spritesheet {
                    draw_monster_sprite(ctx, m, img, sx, sy);
                } else {
                    draw_monster(ctx, m, sx, sy);
                }
            }
        }
    }
}

fn draw_isometric_grid(ctx: &CanvasRenderingContext2d) {
    let logical_width = 2000.0; // Large world
    let logical_height = 2000.0;
    let step = 32.0; // Logical step
    
    ctx.set_stroke_style(&JsValue::from_str("#333333"));
    ctx.set_line_width(0.5);
    
    // Draw vertical lines (logical X lines)
    let mut x = 0.0;
    while x <= logical_width {
        let (sx1, sy1) = to_screen_coord(x, 0.0);
        let (sx2, sy2) = to_screen_coord(x, logical_height);
        ctx.begin_path();
        ctx.move_to(sx1, sy1);
        ctx.line_to(sx2, sy2);
        ctx.stroke();
        x += step;
    }
    
    // Draw horizontal lines (logical Y lines)
    let mut y = 0.0;
    while y <= logical_height {
        let (sx1, sy1) = to_screen_coord(0.0, y);
        let (sx2, sy2) = to_screen_coord(logical_width, y);
        ctx.begin_path();
        ctx.move_to(sx1, sy1);
        ctx.line_to(sx2, sy2);
        ctx.stroke();
        y += step;
    }
}

fn draw_player_sprite(ctx: &CanvasRenderingContext2d, player: &Player, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 16.0;
    let draw_y = sy - 24.0;
    
    // Rows in new spritesheet:
    // 0: Male Warrior
    // 1: Female Warrior (Example mapping)
    // 2: Mage (Example)
    // 3: Skeleton
    // 4: Bat
    // 5: Effects
    
    let row = match (player.gender.as_str(), player.class) {
        ("female", _) => 1.0,
        (_, PlayerClass::Mage) => 2.0, // Prioritize class visuals for mage
        _ => 0.0, // Default Male Warrior/Rogue etc
    };
    
    let src_y = row * 48.0; // Assuming 48px height per character row in the generated sheet (often taller than 32)
                            // or 32 if standard. Let's assume 32px based on previous code or prompt saying 32x48.
                            // User prompt said "32x48 px characters". So height is 48.
    
    // Animation frame logic
    let src_x = if player.is_moving {
        let frame = (js_sys::Date::now() / 200.0) as i32 % 2; // Simple 2 frame walk
        (frame as f64 + 1.0) * 32.0 
    } else if player.is_attacking {
        3.0 * 32.0 // Attack frame
    } else {
        0.0 // Idle
    };
        
    // Draw 32x48 character
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, src_x, src_y, 32.0, 48.0, draw_x, draw_y - 16.0, 32.0, 48.0
    ).unwrap_or_else(|_| ());
    
    // Name
    ctx.set_fill_style(&JsValue::from_str("#c5c6c7"));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&player.username, sx, draw_y - 5.0).unwrap();

    // HP bar
    draw_hp_bar(ctx, sx, draw_y - 8.0, player.combat_stats.hp, player.combat_stats.max_hp);
    
    // Effects overlay (Mockup)
    if player.is_attacking {
        // Draw slash effect from Row 5
        let effect_row = 5.0;
        ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img, 0.0, effect_row * 32.0, 32.0, 32.0, sx - 16.0, sy - 16.0, 32.0, 32.0
        ).unwrap_or_else(|_| ());
    }
}

fn draw_monster_sprite(ctx: &CanvasRenderingContext2d, monster: &Monster, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 16.0;
    let draw_y = sy - 24.0;
    
    // Map monster ID/Name to rows
    let (row, height) = match monster.name.as_str() {
        "박쥐" | "Dark Bat" => (4.0, 32.0),
        _ => (3.0, 48.0), // Skeleton and others default to Row 3
    };
    
    let src_y = row * 48.0; // Base offset. Note: standardizing rows to 48px strides for simplicity?
                            // Or checking prompt. Prompt Row 3: Skeleton, Row 4: Bat.
    
    let src_x = (js_sys::Date::now() / 200.0) as i32 % 2; // Idle anim
    let src_x = (src_x as f64) * 32.0;
    
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, src_x, src_y, 32.0, height, draw_x, draw_y - (height - 32.0), 32.0, height
    ).unwrap_or_else(|_| ());
    
    ctx.set_fill_style(&JsValue::from_str("#ff4444")); // Red name for monsters
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
