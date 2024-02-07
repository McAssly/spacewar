use std::ptr::null_mut;
use raylib_ffi::*;
use crate::rl::vector::*;

pub unsafe fn CheckCollisionLineRect(p1: Vector2, p2: Vector2, rect: Rectangle) -> bool {
    CheckCollisionLines(p1, p2, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.x + rect.width, y: rect.y }, null_mut())
    || CheckCollisionLines(p1, p2, Vector2 { x: rect.x + rect.width, y: rect.y }, Vector2 { x: rect.x + rect.width, y: rect.y + rect.height }, null_mut())
    || CheckCollisionLines(p1, p2, Vector2 { x: rect.x + rect.width, y: rect.y + rect.height }, Vector2 { x: rect.x, y: rect.y + rect.height }, null_mut())
    || CheckCollisionLines(p1, p2, Vector2 { x: rect.x, y: rect.y + rect.height }, Vector2 { x: rect.x, y: rect.y }, null_mut())
}

pub unsafe fn CheckCollisionLineRectEx(p1: Vector2, p2: Vector2, rect: Rectangle, thickness: f32) -> bool {
    let angle: f32 = vector2::angle_from_line(&p1, &p2);
    let modifier: Vector2 = vector2::from_angle(angle, thickness);
    CheckCollisionLineRect(vector2::add(&p1, &modifier), vector2::add(&p2, &modifier), rect)
    || CheckCollisionLineRect(vector2::sub(&p1, &modifier), vector2::sub(&p2, &modifier), rect)
    || CheckCollisionLineRect(p1, p2, rect)
}