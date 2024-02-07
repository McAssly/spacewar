use raylib_ffi::*;
use colors::*;
use crate::delta;
use crate::rl::vector::vector2 as v;

pub enum TextPosition {
    TL,
    TR,
    BL,
    BR,
}

pub unsafe fn draw_text(text_s: String, pos: TextPosition, y_off: i32, font_size: i32, color: Color) {
    let text: &str = text_s.as_str();
    let text_w = MeasureText(rl_str!(text), font_size);
    let padding = 8;
    match pos {
        TextPosition::TL => DrawText(rl_str!(text), padding, font_size * y_off + padding, font_size, color),
        TextPosition::BL => DrawText(rl_str!(text), padding, GetScreenHeight() - font_size * y_off - padding, font_size, color),
        TextPosition::BR => DrawText(rl_str!(text), GetScreenWidth() - padding - text_w, GetScreenHeight() - font_size * y_off - padding, font_size, color),
        TextPosition::TR => DrawText(rl_str!(text), GetScreenWidth() - padding - text_w, font_size * y_off + padding, font_size, color),
    }
}

pub unsafe fn draw_fps(pos: TextPosition, y_off: i32) {
    draw_text(format!("{}", (1.0 / delta!()) as i32), pos, y_off, 20, GREEN); 
}

pub fn fmt_time(seconds: f32) -> String {
    format!("{:02}:{:02}", (seconds / 60.0) as i32, (seconds % 60.0) as i32)
}

pub unsafe fn measure_text_w(text: &String, font_size: i32) -> f32 {
    (MeasureText(rl_str!(text), font_size) + 5) as f32
}

pub unsafe fn measure_text_h(text: &String, font_size: i32) -> f32 {
    MeasureTextEx(GetFontDefault(), rl_str!(text), font_size as f32, 0.0).y + 5.0
}

pub unsafe fn draw_label(text: &str, font_size: i32, x: i32, y: i32, color: Color) {
    let measured = MeasureTextEx(GetFontDefault(), rl_str!(text), font_size as f32, 0.0);
    let width = MeasureText(rl_str!(text), font_size) as i32;
    let height = measured.y as i32;
    DrawText(rl_str!(text), x - width / 2, y - height / 2, font_size, color); 
}

pub struct Button {
    x: f32,
    y: f32,
    font_size: i32,
    color0: Color,
    color1: Color,
    pub label: String, 
    selected: bool,
}

impl Button {
    pub fn new(label: String, x: f32, y: f32, font_size: i32, color0: Color, color1: Color) -> Button {
        Button {
            x, y, font_size, color0, color1, label, selected: false, 
        }
    }

    pub unsafe fn new_center(label: &String, center: &Vector2, font_size: i32, color0: Color, color1: Color) -> Button {
        Button {
            x: center.x - measure_text_w(label, font_size) / 2.0,
            y: center.y - measure_text_h(label, font_size) / 2.0,
            font_size,
            color0,
            color1,
            label: label.to_string(),
            selected: false,
        }
    }

    /// @brief  Creates a row of buttons at the given row position
    /// @note   Screen Percent is the percent from the screen (bottom up)
    pub unsafe fn new_row(screen_percent: f32, labels: Vec<String>, font_size: i32, min: f32, max: f32, color0: Color, color1: Color) -> Vec<Button> {
        let mut buttons: Vec<Button> = vec![];
        let pos_off = (max - min) / (labels.len() + 1) as f32;
        for i in 0..labels.len() {
            buttons.push(
                Button::new_center(
                    &labels[i],
                    &Vector2 {
                        x: pos_off * (i + 1) as f32 + min,
                        y: GetScreenHeight() as f32 - GetScreenHeight() as f32 * screen_percent
                    },
                    font_size, 
                    color0,
                    color1,
                )
            );
        }
        buttons
    }

    pub unsafe fn new_list_centered(y_off: i32, padding: i32, font_size: i32, min: f32, max: f32, color0: Color, color1: Color, labels: Vec<String>) -> Vec<Button> {
        let mut buttons: Vec<Button> = vec![];
        let pad = padding as f32;
        let x = (max - min) / 2.0;
        for i in 0..labels.len() {
            buttons.push(
                Button::new_center(
                    &labels[i],
                    &Vector2 {
                        x, 
                        y: (GetScreenHeight() / 2 + y_off) as f32 + (measure_text_h(&labels[i], font_size) + pad) * i as f32
                    },
                    font_size,
                    color0,
                    color1,
                )
            );
        }
        buttons
    }

    pub unsafe fn rec(&self) -> Rectangle {
        Rectangle {
            x: self.x, y: self.y, width: self.width(), height: self.height()
        }
    }

    pub unsafe fn within(&self, cursor: &Vector2) -> bool {
        v::within(cursor, &self.rec())
    }

    pub unsafe fn center(&self) -> Vector2 {
        Vector2 {
            x: self.x + self.width() / 2.0,
            y: self.y + self.height() / 2.0,
        }
    }

    pub fn select(&mut self, state: bool) {
        self.selected = state;
    }

    pub unsafe fn width(&self) -> f32 {
        measure_text_w(&self.label, self.font_size) + 5.0
    }

    pub unsafe fn height(&self) -> f32 {
        measure_text_h(&self.label, self.font_size) + 5.0
    }

    pub unsafe fn draw(&self, cursor: &Vector2) {
        if v::within(&cursor, &self.rec()) || self.selected {
            DrawRectangleRec(self.rec(), self.color1);
            DrawRectangleLinesEx(self.rec(), 2.0, self.color0);
            DrawText(rl_str!(&self.label), self.x as i32 + 5, self.y as i32 + 5, self.font_size, self.color0); 
            return
        }
        DrawRectangleRec(self.rec(), self.color0);
        DrawText(rl_str!(&self.label), self.x as i32 + 5, self.y as i32 + 5, self.font_size, self.color1); 
    }
}
