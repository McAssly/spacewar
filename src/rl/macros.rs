pub mod macros {
    #[macro_export]
    macro_rules! delta {
        // kinda pointless, but I like it
        () => {
            GetFrameTime()
        };
    }

    #[macro_export]
    macro_rules! key {
        ($variant:ident) => {
            enums::KeyboardKey::$variant as i32
        };
    }

    #[macro_export]
    macro_rules! get_input {
        ($up:ident, $left:ident, $down:ident, $right:ident) => {
            Vector2 {
                x: (IsKeyDown(enums::KeyboardKey::$right as i32) as i8 - IsKeyDown(enums::KeyboardKey::$left as i32) as i8) as f32,
                y: (IsKeyDown(enums::KeyboardKey::$down as i32) as i8 - IsKeyDown(enums::KeyboardKey::$up as i32) as i8) as f32,
            }
        };
    }

    #[macro_export]
    macro_rules! mouse_b {
        ($variant:ident) => {
            enums::MouseButton::$variant as i32
        };
    }
}

