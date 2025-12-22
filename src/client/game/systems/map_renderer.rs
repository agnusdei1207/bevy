use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::shared::domain::map::{MapData, TileType, MapObject, ObjectType};
use crate::shared::domain::shared::models::Position;

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

    pub fn to_screen(&self, grid_x: f64, grid_y: f64) -> (f64, f64) {
        let sx = self.origin_x + (grid_x - grid_y) * (self.tile_width / 2.0);
        let sy = self.origin_y + (grid_x + grid_y) * (self.tile_height / 2.0);
        (sx, sy)
    }

    pub fn render(
        &self, 
        ctx: &CanvasRenderingContext2d, 
        map_data: &MapData, 
        tile_sheet: &Option<HtmlImageElement>,
        building_sheet: &Option<HtmlImageElement>,
        lamp_sheet: &Option<HtmlImageElement>
    ) {
        // Render Tiles (Ground)
        if let Some(sheet) = tile_sheet {
            for gy in 0..map_data.height {
                for gx in 0..map_data.width {
                    let tile = &map_data.tiles[gy][gx];
                    // Skip Water/Wall if handled as object? No, tile is base.
                    
                    let (sx, sy) = self.to_screen(gx as f64, gy as f64);
                    
                    let src_x = match tile {
                        TileType::Grass => 0.0,
                        TileType::Stone => 64.0,
                        TileType::Water => 128.0, 
                        TileType::Wall => 192.0, // Should be an object usually, but can be tile block
                        TileType::Portal => 256.0, 
                    };
                    
                    // Draw Tile
                    // Adjust drawing so center aligns.
                    // Image tile size is 64x48 (taller)
                    let draw_w = 64.0;
                    let draw_h = 48.0;
                    let dx = sx - draw_w / 2.0;
                    let dy = sy - (draw_h - self.tile_height / 2.0);
                    
                    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        sheet, src_x, 0.0, draw_w, draw_h, dx, dy, draw_w, draw_h
                    ).unwrap_or(());
                }
            }
        }
        
        // Render Objects (Buildings, etc.) - Sorted by Y for depth
        // We need to mix entities here ideally, but for now drawn after floor.
        if let Some(sheet) = building_sheet {
            let mut objects = map_data.objects.clone();
            objects.sort_by(|a, b| (a.x + a.y).total_cmp(&(b.x + b.y)));
            
            for obj in objects {
                let (sx, sy) = self.to_screen(obj.x, obj.y);
                
                match obj.obj_type {
                    ObjectType::Lamp => {
                        if let Some(l_sheet) = lamp_sheet {
                             // Use separate lamp sheet
                             // Single asset 32x64 or similar?
                             // Generated asset is tall. Let's assume 32x64 fit.
                             let w = 32.0;
                             let h = 96.0; // Tall torch
                             let dx = sx - w / 2.0;
                             let dy = sy - 80.0; // Anchor bottom
                             
                             ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                l_sheet, 0.0, 0.0, w, h, dx, dy, w, h
                             ).unwrap_or(());
                        }
                    },
                    _ => {
                        // Use building sheet
                        let (src_x, w, h, offset_y) = match obj.obj_type {
                            ObjectType::House => (0.0, 128.0, 128.0, 96.0),
                            ObjectType::Blacksmith => (128.0, 128.0, 128.0, 96.0),
                            ObjectType::Tavern => (256.0, 128.0, 128.0, 96.0),
                            ObjectType::GuildHall => (384.0, 128.0, 128.0, 96.0),
                            ObjectType::Lamp => (0.0, 0.0, 0.0, 0.0), // Handled above
                        };
                        
                        let dx = sx - w / 2.0;
                        let dy = sy - offset_y; 
                        
                        ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            sheet, src_x, 0.0, w, h, dx, dy, w, h
                        ).unwrap_or(());
                    }
                }
            }
        }
    }
}
