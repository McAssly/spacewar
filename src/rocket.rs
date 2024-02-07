use raylib_ffi::*;
use colors::*;
use crate::{rl::vector::vector2 as v, ship::Ship};
use std::f32::consts::PI;

pub struct Rocket {
    pub center: Vector2,
    pub velocity: Vector2,
    pub angle: f32,
    pub life: f32,
}

impl Rocket {
    pub unsafe fn new(center: Vector2, angle: f32) -> Rocket {
        Rocket { 
            center,
            velocity: v::zero(),
            angle,
            life: 1.0,
        }
    }

    pub unsafe fn update(&mut self, main_star: &Vector2, delta: f32) {
        let accel = 5.0;
        self.velocity = v::add(&self.velocity, &v::from_angle(self.angle, accel));
        self.velocity = v::add(&self.velocity, &v::gravitate(&self.center, &main_star));
        self.center = v::add(&self.center, &v::scale(&self.velocity, delta));
        self.center = v::wrap(&self.center, GetScreenWidth() as f32, GetScreenHeight() as f32);
        self.life -= delta;
    }

    pub unsafe fn draw(&self) {
        let angle_off = PI / 12.0;
        let length = 10.0; // half of actual
        let p0 = v::add(&self.center, &v::from_angle(self.angle + angle_off, length));
        let p1 = v::add(&self.center, &v::from_angle(self.angle - angle_off, length));
        let p2 = v::sub(&self.center, &v::from_angle(self.angle + angle_off, length));
        let p3 = v::sub(&self.center, &v::from_angle(self.angle - angle_off, length));
        DrawLineV(p0, p1, WHITE);
        DrawLineV(p1, p2, WHITE);
        DrawLineV(p2, p3, WHITE);
        DrawLineV(p3, p0, WHITE);
    }

    pub unsafe fn collide_with(&self, ship: &Ship) -> bool {
        let angle_off = PI / 12.0;
        let length = 10.0; // half of actual
        let p0 = v::add(&self.center, &v::from_angle(self.angle + angle_off, length));
        let p1 = v::add(&self.center, &v::from_angle(self.angle - angle_off, length));
        let p2 = v::sub(&self.center, &v::from_angle(self.angle + angle_off, length));
        let p3 = v::sub(&self.center, &v::from_angle(self.angle - angle_off, length));
        ship.line_hits(p0, p1) ||
        ship.line_hits(p1, p2) ||
        ship.line_hits(p2, p3) ||
        ship.line_hits(p3, p0)
    }
}
