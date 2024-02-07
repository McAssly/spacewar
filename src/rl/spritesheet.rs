use raylib_ffi::*;
use colors::*;

use super::vector::vector2;

pub struct Sprite {
    sprite: Texture2D,
    frame_count: i32,
    frames: Vec<Rectangle>,
    pub frame: usize,
    pub iframe: f32,
    pub frame_rate: f32,
}

impl Sprite {
    pub unsafe fn new(fileloc: &str) -> Sprite {
        let sprite: Texture2D = LoadTexture(rl_str!(fileloc));
        Sprite {
            sprite,
            frame_count: 0,
            frames: vec![],
            frame: 0,
            iframe: 0.0,
            frame_rate: 0.0,
        }
    }

    pub unsafe fn new_anim(fileloc: &str, pw: i32, ph: i32, frame_count: i32, frame_rate: f32) -> Sprite {
        let sprite: Texture2D = LoadTexture(rl_str!(fileloc));
        let mut frames: Vec<Rectangle> = vec![];
        for y in 0..(sprite.height / ph) {
            for x in 0..(sprite.width / pw) {
                frames.push(Rectangle {
                    x: (x * pw) as f32,
                    y: (y * ph) as f32,
                    width: pw as f32,
                    height: ph as f32,
                });
            }
        }
        Sprite {
            sprite,
            frame_count,
            frames,
            frame: 0,
            iframe: 0.0,
            frame_rate,
        }
    }

    pub unsafe fn draw(&self, p: Vector2) {
        if self.frames.len() == 0 {
            DrawTexture(self.sprite, p.x as i32, p.y as i32, WHITE);
            return;
        }
        DrawTextureRec(self.sprite, self.frames[self.frame], p, WHITE);
    }

    pub unsafe fn draw_flipped_h(&self, p: Vector2) {
        let src = Rectangle {
            x: p.x,
            y: p.y,
            width: self.width() as f32,
            height: self.height() as f32,
        };
        if self.frames.len() == 0 {
            let mut full = Rectangle {
                x: 0.0, y: 0.0,
                width: self.sprite.width as f32,
                height: self.sprite.height as f32,
            };
            full.x += full.width;
            full.width *= -1.0;
            DrawTexturePro(self.sprite, full, src, vector2::zero(), 0.0, WHITE);
            return;
        }
        let mut frame = self.frames[self.frame];
        frame.x += frame.width;
        frame.width *= -1.0;
        DrawTexturePro(self.sprite, self.frames[self.frame], src, vector2::zero(), 0.0, WHITE);
    }

    /// @brief  Used for syncing other sprites with one another
    pub unsafe fn draw_ahead(&self, p: Vector2, frames: usize) {
        if self.frames.len() == 0 {
            self.draw(p);
            return
        }
        let aheadi: f32 = self.iframe + 1.0 / self.frame_rate * frames as f32;
        let mut ahead: usize = (aheadi * self.frame_rate) as usize;
        if ahead >= self.frame_count as usize {
            ahead -= self.frame_count as usize;
        }
        DrawTextureRec(self.sprite, self.frames[ahead], p, WHITE);
    }

    pub fn animate(&mut self, delta: f32) {
        if self.frames.len() == 0 { return }
        self.iframe += delta;
        self.frame = (self.iframe * self.frame_rate) as usize;
        if self.frame as i32 >= self.frame_count {
            self.frame = 0;
            self.iframe = 0.0;
        }
    }

    /// @brief  The total duration the animation takes
    pub fn duration(&self) -> f32 {
        self.frame_count as f32 / self.frame_rate
    }

    pub fn width(&self) -> i32 {
        if self.frames.len() == 0 {
            return self.sprite.width
        }
        self.frames[0].width as i32
    }

    pub fn height(&self) -> i32 {
        if self.frames.len() == 0 {
            return self.sprite.height
        }
        self.frames[0].height as i32
    }
}
