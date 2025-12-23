//! 맵 렌더러 - 아이소메트릭 타일 기반 맵 렌더링
//!
//! 90년대 RPG 스타일 다크 판타지 맵 시스템
//!
//! hydrate feature가 활성화된 경우에만 렌더링 기능 사용

#[cfg(feature = "csr")]
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::shared::domain::map::{MapData, TileType, MapObject, ObjectType};

/// 아이소메트릭 맵 렌더러
pub struct MapRenderer {
    tile_width: f64,
    tile_height: f64,
    origin_x: f64,
    origin_y: f64,
}

impl MapRenderer {
    pub fn new(canvas_width: f64, _canvas_height: f64) -> Self {
        Self {
            tile_width: 64.0,
            tile_height: 32.0,
            origin_x: canvas_width / 2.0,
            origin_y: 100.0,
        }
    }

    /// 그리드 좌표를 화면 좌표로 변환
    #[allow(dead_code)]
    pub fn to_screen(&self, grid_x: f64, grid_y: f64) -> (f64, f64) {
        let sx = self.origin_x + (grid_x - grid_y) * (self.tile_width / 2.0);
        let sy = self.origin_y + (grid_x + grid_y) * (self.tile_height / 2.0);
        (sx, sy)
    }

    /// 맵 렌더링 (hydrate only)
    #[cfg(feature = "csr")]
    pub fn render(
        &self, 
        ctx: &CanvasRenderingContext2d, 
        map_data: &MapData, 
        tileset: Option<&HtmlImageElement>,
        buildings: Option<&HtmlImageElement>,
        decorations: Option<&HtmlImageElement>
    ) {
        // 바닥 타일 렌더링
        if let Some(sheet) = tileset {
            for gy in 0..map_data.height {
                for gx in 0..map_data.width {
                    let tile = &map_data.tiles[gy][gx];
                    let (sx, sy) = self.to_screen(gx as f64, gy as f64);
                    
                    let src_x = match tile {
                        TileType::Grass => 0.0,
                        TileType::Stone => 64.0,
                        TileType::Water => 128.0, 
                        TileType::Wall => 192.0,
                        TileType::Portal => 256.0, 
                    };
                    
                    let draw_w = 64.0;
                    let draw_h = 48.0;
                    let dx = sx - draw_w / 2.0;
                    let dy = sy - (draw_h - self.tile_height / 2.0);
                    
                    let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        sheet, src_x, 0.0, draw_w, draw_h, dx, dy, draw_w, draw_h
                    );
                }
            }
        } else {
            // 대체 렌더링
            for gy in 0..map_data.height {
                for gx in 0..map_data.width {
                    let tile = &map_data.tiles[gy][gx];
                    let (sx, sy) = self.to_screen(gx as f64, gy as f64);
                    
                    let color = match tile {
                        TileType::Grass => "#1a2e1a",
                        TileType::Stone => "#2d3748",
                        TileType::Water => "#16213e",
                        TileType::Wall => "#1a1a2e",
                        TileType::Portal => "#4a0080",
                    };
                    
                    self.draw_iso_tile(ctx, sx, sy, color);
                }
            }
        }
        
        // 오브젝트 렌더링
        let mut objects = map_data.objects.clone();
        objects.sort_by(|a, b| (a.x + a.y).total_cmp(&(b.x + b.y)));
        
        for obj in objects {
            let (sx, sy) = self.to_screen(obj.x, obj.y);
            
            match obj.obj_type {
                ObjectType::Lamp => {
                    if let Some(deco_sheet) = decorations {
                        let w = 32.0;
                        let h = 64.0;
                        let dx = sx - w / 2.0;
                        let dy = sy - h + 16.0;
                        
                        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            deco_sheet, 0.0, 0.0, w, h, dx, dy, w, h
                        );
                    } else {
                        self.draw_torch_placeholder(ctx, sx, sy);
                    }
                },
                _ => {
                    if let Some(building_sheet) = buildings {
                        let (src_x, w, h, offset_y) = match obj.obj_type {
                            ObjectType::House => (0.0, 128.0, 128.0, 96.0),
                            ObjectType::Blacksmith => (128.0, 128.0, 128.0, 96.0),
                            ObjectType::Tavern => (256.0, 128.0, 128.0, 96.0),
                            ObjectType::GuildHall => (384.0, 128.0, 128.0, 96.0),
                            ObjectType::Lamp => (0.0, 0.0, 0.0, 0.0),
                        };
                        
                        if w > 0.0 {
                            let dx = sx - w / 2.0;
                            let dy = sy - offset_y;
                            
                            let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                building_sheet, src_x, 0.0, w, h, dx, dy, w, h
                            );
                        }
                    } else {
                        self.draw_building_placeholder(ctx, sx, sy, &obj.obj_type);
                    }
                }
            }
        }
    }

    #[cfg(feature = "csr")]
    fn draw_iso_tile(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, color: &str) {
        let hw = self.tile_width / 2.0;
        let hh = self.tile_height / 2.0;
        
        ctx.begin_path();
        ctx.move_to(x, y - hh);
        ctx.line_to(x + hw, y);
        ctx.line_to(x, y + hh);
        ctx.line_to(x - hw, y);
        ctx.close_path();
        ctx.set_fill_style_str(color);
        ctx.fill();
        
        ctx.set_stroke_style_str("#0a0a0a");
        ctx.set_line_width(0.5);
        ctx.stroke();
    }

    #[cfg(feature = "csr")]
    fn draw_torch_placeholder(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
        ctx.set_fill_style_str("#2d3748");
        ctx.fill_rect(x - 2.0, y - 20.0, 4.0, 20.0);
        
        ctx.begin_path();
        let _ = ctx.arc(x, y - 24.0, 6.0, 0.0, std::f64::consts::PI * 2.0);
        ctx.set_fill_style_str("#8b7355");
        ctx.fill();
    }

    #[cfg(feature = "csr")]
    fn draw_building_placeholder(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, obj_type: &ObjectType) {
        let (width, height, color) = match obj_type {
            ObjectType::House => (40.0, 50.0, "#1a2e1a"),
            ObjectType::Blacksmith => (50.0, 45.0, "#2d3748"),
            ObjectType::Tavern => (45.0, 40.0, "#3d1a4f"),
            ObjectType::GuildHall => (60.0, 60.0, "#16213e"),
            ObjectType::Lamp => return,
        };
        
        ctx.set_fill_style_str(color);
        ctx.fill_rect(x - width / 2.0, y - height, width, height);
        
        ctx.begin_path();
        ctx.move_to(x - width / 2.0 - 5.0, y - height);
        ctx.line_to(x, y - height - 20.0);
        ctx.line_to(x + width / 2.0 + 5.0, y - height);
        ctx.close_path();
        ctx.set_fill_style_str("#660000");
        ctx.fill();
        
        ctx.set_stroke_style_str("#0a0a0a");
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x - width / 2.0, y - height, width, height);
    }

    /// 테스트용 모의 맵 생성
    pub fn create_mock_map() -> MapData {
        let width = 10;
        let height = 10;
        
        let mut tiles = vec![vec![TileType::Grass; width]; height];
        
        for tile in tiles[height / 2].iter_mut().take(width) {
            *tile = TileType::Stone;
        }
        for row in tiles.iter_mut().take(height) {
            row[width / 2] = TileType::Stone;
        }
        
        tiles[2][7] = TileType::Water;
        tiles[2][8] = TileType::Water;
        tiles[3][7] = TileType::Water;
        
        tiles[0][0] = TileType::Wall;
        tiles[0][9] = TileType::Wall;
        tiles[9][0] = TileType::Wall;
        tiles[9][9] = TileType::Wall;
        
        tiles[1][1] = TileType::Portal;
        
        let objects = vec![
            MapObject { id: "house1".to_string(), obj_type: ObjectType::House, x: 2.0, y: 2.0 },
            MapObject { id: "blacksmith".to_string(), obj_type: ObjectType::Blacksmith, x: 7.0, y: 2.0 },
            MapObject { id: "tavern".to_string(), obj_type: ObjectType::Tavern, x: 2.0, y: 7.0 },
            MapObject { id: "guild".to_string(), obj_type: ObjectType::GuildHall, x: 7.0, y: 7.0 },
            MapObject { id: "lamp1".to_string(), obj_type: ObjectType::Lamp, x: 4.0, y: 4.0 },
            MapObject { id: "lamp2".to_string(), obj_type: ObjectType::Lamp, x: 6.0, y: 4.0 },
            MapObject { id: "lamp3".to_string(), obj_type: ObjectType::Lamp, x: 4.0, y: 6.0 },
            MapObject { id: "lamp4".to_string(), obj_type: ObjectType::Lamp, x: 6.0, y: 6.0 },
        ];
        
        MapData { width, height, tiles, objects }
    }
}
