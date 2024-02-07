use raylib_ffi::*;
use colors::*;

pub struct Tileset {
    sheet: Texture2D,
    tiles: Vec<Rectangle>,
    tile_size: i32,
}

impl Tileset {
    pub unsafe fn new(fileloc: &str, tile_size: i32) -> Tileset {
        let sheet: Texture2D = LoadTexture(rl_str!(fileloc));
        let mut tiles: Vec<Rectangle> = vec![];
        let width = sheet.width / tile_size;
        let height = sheet.height / tile_size;
        for y in 0..height {
            for x in 0..width {
                tiles.push(Rectangle {
                    x: (x * tile_size) as f32,
                    y: (y * tile_size) as f32,
                    width: tile_size as f32,
                    height: tile_size as f32,
                });
            }
        }
        Tileset {
            sheet,
            tiles,
            tile_size,
        }
    }

    pub unsafe fn draw_tile(&self, position: Vector2, index: usize) {
        DrawTextureRec(self.sheet, self.tiles[index], position, WHITE);
    }

    pub unsafe fn draw(&self, position: Vector2) {
        DrawTexture(self.sheet, position.x as i32, position.y as i32, WHITE);
    }
}
