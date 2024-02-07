use std::{ffi::{c_void, c_int}, mem};

use raylib_ffi::*;
use colors::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use rl::{gui::*, vector::vector2};
use ship::{Ship, Player};

pub mod rl {
    pub mod collision;
    pub mod extras;
    pub mod macros;
    pub mod spritesheet;
    pub mod tileset;
    pub mod timer;
    pub mod vector;
    pub mod gui;
}

mod ship;
mod rocket;

unsafe fn draw_stars(seed: u32) -> RenderTexture2D {
    let scale: f64 = 0.3;               // scale is inverse, 0 means larger 1 means bigger
    let dither = 2;                     // the amount of pixels to dither + 1
    let threshold: f64 = 0.99;
    let perlin = Perlin::new(seed);
    
    let width = GetScreenWidth();
    let height = GetScreenHeight();

    let render = LoadRenderTexture(width, height);

    BeginTextureMode(render);
    ClearBackground(BLACK);

    for x in 0..width {
        for y in 0..height {
            let value = perlin.get([(x as f64) * scale, (y as f64) * scale]);
            let alpha_t = ((value + 1.0) / 2.0).clamp(0.0,1.0);
            let alpha = if alpha_t > threshold && (x*y) % dither == 0 {
                (255.0 * ((alpha_t - threshold) / (1.0 - threshold))) as u8
            } else {
                0
            };
            DrawPixel(x, y, Color {r:255,g:255,b:255,a:alpha});
        }
    }

    EndTextureMode();
    render
}

enum GameState {
    Menu,
    Game,
}

unsafe fn convert_cvoid(value: f32) -> *const c_void {
    let bytes: [u8; 4] = mem::transmute_copy(&value);
    bytes.as_ptr() as *const c_void
}

fn main() { unsafe {
    InitWindow(512, 512, rl_str!("Spacewar!"));
    let shader = LoadShader(rl_str!("base.vs"), rl_str!("scanlines.fs"));
    let target = LoadRenderTexture(512, 512);
    SetTargetFPS(144);
    SetExitKey(0);
    let mut should_exit = false;
    let mut stars = draw_stars(rand::thread_rng().gen());
    let mut game_state = GameState::Menu;

    // set our menu data
    let mut menu_selected: i32 = -1;
    let mut menu = Button::new_list_centered(
        -10, 20, 30, 0.0, GetScreenHeight() as f32, WHITE, BLACK,
        vec![
            "1 Player".to_string(),
            "2 Player".to_string(),
            "Quit".to_string(),
        ]
    );

    // set our game data
    let main_star = Vector2 { x: (GetScreenWidth() / 2) as f32, y: (GetScreenHeight() / 2) as f32 };
    let mut p1 = Ship::new(ship::Player::One);
    let mut p2 = Ship::new(ship::Player::Rob);

    let mut ms = 0.0;
    let time_loc = GetShaderLocation(shader, rl_str!("time"));

    while !should_exit { 
        if WindowShouldClose() {
            should_exit = true;
        }
        let delta = delta!();
        let cursor = GetMousePosition();
        if IsKeyPressed(key!(F1)) { stars = draw_stars(rand::thread_rng().gen()); }
        SetShaderValue(shader, time_loc, convert_cvoid(ms), enums::ShaderUniformDataType::Float as c_int);
        ms += delta;

        match &game_state {
            GameState::Menu => {
                let prev_selected = menu_selected;
                if IsKeyPressed(key!(Up)) || IsKeyPressed(key!(W)) {
                    menu_selected -= 1;
                    if menu_selected < 0 {
                        menu_selected = menu.len() as i32 - 1;
                    }
                }
                if IsKeyPressed(key!(Down)) || IsKeyPressed(key!(S)) {
                    menu_selected += 1;
                    if menu_selected >= menu.len() as i32 {
                        menu_selected = 0;
                    }
                }
                if IsKeyPressed(key!(Escape)) {
                    menu_selected = -1;
                }
                if menu_selected != prev_selected {
                    for i in 0..menu.len() {
                        menu[i].select(false);
                    }
                }
                if menu_selected != -1 {
                    let k_selected = &mut menu[menu_selected as usize];
                    k_selected.select(true);
                    if IsKeyPressed(key!(Enter)) || IsKeyPressed(key!(Space)) {
                        match k_selected.label.as_str() {
                            "1 Player" => {
                                game_state = GameState::Game;
                                p2 = Ship::new(ship::Player::Rob);
                            }
                            "2 Player" => {
                                game_state = GameState::Game;
                                p2 = Ship::new(ship::Player::Two);
                            },
                            "Quit" => should_exit = true,
                            _ => {} 
                        }
                    }
                }
                // mouse handler
                /*
                let selected = menu.iter().find(
                    |b| b.within(&cursor) && IsMouseButtonPressed(mouse_b!(Left)));
                if selected.is_some() { 
                    match selected.unwrap().label.as_str() {
                        "1 Player" => game_state = GameState::Game,
                        "2 Player" => game_state = GameState::Game,
                        "Quit" => should_exit = true,
                        _ => {}
                    }
                }
                */
            },
            GameState::Game => {
                if IsKeyPressed(key!(Backspace)) || IsKeyPressed(key!(Escape)) {
                    game_state = GameState::Menu;
                    p1 = Ship::new(ship::Player::One);
                    p2 = Ship::new(ship::Player::Rob);
                }
                if !p1.dead { p1.update(&main_star, &p2, delta); }
                if !p2.dead { p2.update(&main_star, &p1, delta); }
            }
        }
        
        BeginTextureMode(target);
            ClearBackground(BLACK);
            DrawTexture(stars.texture, 0, 0, WHITE);
            match &game_state {
                GameState::Menu => {
                    menu.iter().for_each(|b| b.draw(&cursor));
                    draw_label("Spacewar!", 60, GetScreenWidth() / 2, GetScreenHeight() / 2 - 80, WHITE);
                },
                GameState::Game => {
                    DrawCircleV(main_star, 10.0, WHITE);
                    p1.draw();
                    p2.draw();
                    match p2.player {
                        Player::Rob => {
                            if p1.dead {
                                draw_label("Game Over.", 60, GetScreenWidth() / 2, GetScreenHeight() / 2, RED);
                            }
                            else if p2.dead {
                                draw_label("You Win!", 60, GetScreenWidth() / 2, GetScreenHeight() / 2, GREEN);
                            }
                        },
                        _ => {
                            if p1.dead {
                                draw_label("Player 2 Wins!", 60, GetScreenWidth() / 2, GetScreenHeight() / 2, GOLD);
                            } else if p2.dead {
                                draw_label("Player 1 Wins!", 60, GetScreenWidth() / 2, GetScreenHeight() / 2, GREEN);
                            }
                        }
                    }
                }
            }
        EndTextureMode();

        BeginDrawing();
        ClearBackground(BLACK);
        BeginShaderMode(shader);
        DrawTextureRec(
            target.texture, 
            Rectangle {
                x: 0.0, y: 0.0,
                width: 512.0,
                height: -512.0,
            },
            vector2::zero(),
            WHITE
        );
        EndShaderMode();
        draw_fps(TextPosition::TL, 0);
        EndDrawing();
    }
    UnloadShader(shader);
    CloseWindow();
}}
