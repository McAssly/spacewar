use raylib_ffi::*;
use colors::*;
use crate::rl::timer::Timer;
use crate::rl::vector::vector2 as v;
use crate::get_input;
use crate::key;
use crate::rocket::Rocket;
use std::f32::consts::PI;
use rand::Rng;
use core::ptr::null_mut;

pub enum Player {
    One,
    Two,
    Rob,
}

pub struct Ship {
    pub player: Player,
    pub center: Vector2,
    velocity: Vector2,
    move_angle: f32,
    view_angle: f32,
    view_mode: bool,
    pub rockets: Vec<Rocket>,
    rob_timer: Timer,
    rocket_cooldown: Timer,
    input_vector: Vector2,
    pub dead: bool,
}

impl Ship {
    pub unsafe fn new(player: Player) -> Ship {
        Ship {
            center: match player {
                Player::One => Vector2 { 
                    x: (GetScreenWidth() / 2 - 56) as f32, 
                    y: (GetScreenHeight() / 2 - 56) as f32
                },
                _ => Vector2 {
                    x: (GetScreenWidth() / 2 + 56) as f32, 
                    y: (GetScreenHeight() / 2 + 56) as f32
                },
            },
            player,
            velocity: v::zero(),
            move_angle: 0.0,
            view_angle: 0.0,
            view_mode: true,
            rockets: vec![],
            rob_timer: Timer::new(0.0),
            rocket_cooldown: Timer::new(0.5),
            input_vector: v::zero(),
            dead: false,
        }
    }

    pub unsafe fn rob_input(&mut self, main_star: &Vector2) -> Vector2 {
        self.rob_timer.start_override(rand::thread_rng().gen_range(0.5..3.0));
        if v::dist(&self.center, main_star) <= 100.0 && self.input_vector.y >= 0.0 {
            return Vector2 {
                x: if self.input_vector.x == 1.0 { -1.0 } else { 1.0 },
                y: rand::thread_rng().gen_range(-1.0..-0.5),
            }
        }
        Vector2 {
            x: if self.input_vector.x == 1.0 { -1.0 } else { 1.0 },
            y: rand::thread_rng().gen_range(-1.0..1.0)
        }
    }

    pub unsafe fn update(&mut self, main_star: &Vector2, other: &Ship, delta: f32) {
        self.rob_timer.update(delta);
        self.rocket_cooldown.update(delta);
        let switch_view_angle = match self.player {
            Player::One => IsKeyPressed(key!(Q)),
            Player::Two => IsKeyPressed(key!(RightShift)),
            Player::Rob => !self.rob_timer.is_running(), 
        };
        let shoot_rocket = !self.rocket_cooldown.is_running() 
            && match self.player {
            Player::One => IsKeyPressed(key!(E)),
            Player::Two => IsKeyPressed(key!(End)),
            Player::Rob => {
                let dist = v::dist(&self.center, &other.center);
                self.in_sight(dist / 360.0 * PI / 6.0, &other) 
                    || self.in_sight(-PI / 6.0 * dist / 360.0, &other) 
            },
        };
        if switch_view_angle { 
            self.view_mode = !self.view_mode;
        }
        if shoot_rocket {
            self.rockets.push(Rocket::new(v::add(&self.center, &v::from_angle(self.view_angle, 25.0)), self.view_angle));
            self.rocket_cooldown.start();
        }
        self.input_vector = match self.player {
            Player::One => get_input!(W, A, S, D),
            Player::Two => get_input!(Up, Left, Down, Right),
            Player::Rob => 
                if v::is_zero(&self.input_vector) || !self.rob_timer.is_running() {
                    self.rob_input(main_star)
                } else {
                    self.input_vector 
                }
            ,
        };

        if self.input_vector.x != 0.0 {
            self.move_angle = v::angle_from_line(&self.center, &main_star) - PI / 2.0 * self.input_vector.x;
            self.velocity = v::from_angle(self.move_angle, 100.0); 
        }
        if self.input_vector.y != 0.0 {
            self.move_angle += self.input_vector.y * PI / 16.0;
            self.velocity = v::from_angle(self.move_angle, 100.0);
        }
        if !self.view_mode { 
            self.view_angle = v::to_angle(&self.velocity);
        }
        self.velocity = v::add(&self.velocity, &v::gravitate(&self.center, &main_star));
        self.center = v::add(&self.center, &v::scale(&self.velocity, delta));
        self.center = v::wrap(&self.center, GetScreenWidth() as f32, GetScreenHeight() as f32);

        if self.view_mode {
            self.view_angle = v::angle_from_line(&self.center, &main_star);
        }

        if CheckCollisionPointCircle(self.center, *main_star, 5.0) 
            || other.rockets.iter().any(|r| r.collide_with(self)) {
            self.dead = true;
        }

        for i in 0..self.rockets.len() {
            self.rockets[i].update(main_star, delta);
        }
        self.rockets.retain(|r| r.life > 0.0);
    }

    pub unsafe fn draw(&self) {
        let point0 = v::add(&self.center, &v::from_angle(self.view_angle, 15.0));
        let point1 = v::add(&self.center, &v::from_angle(self.view_angle + 3.0 * PI / 4.0, 10.0));
        let point2 = v::add(&self.center, &v::from_angle(self.view_angle + 5.0 * PI / 4.0, 10.0));
        let color = match self.player {
            Player::One => GREEN,
            Player::Two => GOLD,
            Player::Rob => RED, 
        };
        DrawCircleV(self.center, 2.0, color);
        DrawLineV(point0, point1, WHITE);
        DrawLineV(point1, point2, WHITE);
        DrawLineV(point2, point0, WHITE);
        self.rockets.iter().for_each(|r| r.draw());
    }

    pub unsafe fn draw_sight(&self, angle_off: f32) {
        let view_point = v::add(&self.center, &v::from_angle(self.view_angle + angle_off, 500.0));
        DrawLineV(self.center, view_point, YELLOW);
    }

    pub unsafe fn in_sight(&self, angle_off: f32, other: &Ship) -> bool {
        let view_point = v::add(&self.center, &v::from_angle(self.view_angle + angle_off, 500.0));
        let point0 = v::add(&other.center, &v::from_angle(other.view_angle, 15.0));
        let point1 = v::add(&other.center, &v::from_angle(other.view_angle + 3.0 * PI / 4.0, 10.0));
        let point2 = v::add(&other.center, &v::from_angle(other.view_angle + 5.0 * PI / 4.0, 10.0));
        CheckCollisionLines(self.center, view_point, point0, point1, null_mut())
            || CheckCollisionLines(self.center, view_point, point1, point2, null_mut())
            || CheckCollisionLines(self.center, view_point, point2, point0, null_mut())
    }

    pub unsafe fn line_hits(&self, start: Vector2, end: Vector2) -> bool {
        let point0 = v::add(&self.center, &v::from_angle(self.view_angle, 15.0));
        let point1 = v::add(&self.center, &v::from_angle(self.view_angle + 3.0 * PI / 4.0, 10.0));
        let point2 = v::add(&self.center, &v::from_angle(self.view_angle + 5.0 * PI / 4.0, 10.0));
        CheckCollisionLines(start, end, point0, point1, null_mut())
            || CheckCollisionLines(start, end, point1, point2, null_mut())
            || CheckCollisionLines(start, end, point2, point0, null_mut())
    }
}
