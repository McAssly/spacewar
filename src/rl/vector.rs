pub mod vector2 {
    use raylib_ffi::{Vector2, Rectangle};

    pub fn zero() -> Vector2 {
        Vector2{x:0.0, y:0.0}
    }

    pub fn to_angle(v: &Vector2) -> f32 {
        v.y.atan2(v.x)
    }

    pub fn from_angle(a: f32, d: f32) -> Vector2 {
        Vector2 { x: d * a.cos(), y: d * a.sin() }
    }

    pub fn add(v1: &Vector2, v2: &Vector2) -> Vector2 {
        Vector2 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
        }
    }

    pub fn add_r(v: &Vector2, d: f32, a: f32) -> Vector2 {
        Vector2 { 
            x: v.x + (d * a.cos()), 
            y: v.y + (d * a.sin()) 
        }
    }

    pub fn sub(v1: &Vector2, v2: &Vector2) -> Vector2 {
        Vector2 {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
        }
    }

    pub fn subf(v: &Vector2, f: f32) -> Vector2 {
        Vector2 { 
            x: v.x - f, 
            y: v.y - f 
        }
    }

    pub fn scale(v: &Vector2, s: f32) -> Vector2 {
        Vector2 {
            x: v.x * s,
            y: v.y * s,
        }
    }

    pub fn divs(v: &Vector2, s: f32) -> Vector2 {
        Vector2 {
            x: v.x / s,
            y: v.y / s,
        }
    }

    pub fn rotate(v: &Vector2, a: f32) -> Vector2 {
        from_angle(v.y.atan2(v.x) + a, len(v))
    }

    pub fn len(v: &Vector2) -> f32 {
        (v.x * v.x + v.y * v.y).sqrt()
    }

    pub fn dist(v1: &Vector2, v2: &Vector2) -> f32 {
        ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2)).sqrt()
    }

    pub fn dist_v(v1: &Vector2, v2: &Vector2) -> Vector2 {
        Vector2 {
            x: v2.x - v1.x,
            y: v2.y - v1.y,
        }
    }

    pub fn norm(v: &Vector2) -> Vector2 {
        Vector2 {
            x: v.x / len(v),
            y: v.y / len(v),
        }
    }

    pub fn floor(v: &Vector2) -> Vector2 {
        Vector2 {
            x: v.x.floor(),
            y: v.y.floor(),
        }
    }

    pub fn round(v: &Vector2) -> Vector2 {
        Vector2 {
            x: v.x.round(),
            y: v.y.round(),
        }
    }

    pub fn angle_from_line(start: &Vector2, end: &Vector2) -> f32 {
        (end.y - start.y).atan2(end.x - start.x)
    }

    pub fn within(v: &Vector2, r: &Rectangle) -> bool {
        v.x >= r.x && v.x <= r.x + r.width && v.y >= r.y && v.y <= r.y + r.height
    }

    pub fn is_zero(v: &Vector2) -> bool {
        v.x == 0.0 && v.y == 0.0
    }

    pub fn eq(v1: &Vector2, v2: &Vector2) -> bool {
        v1.x == v2.x && v1.y == v2.y
    }

    pub fn eq_floor(v1: &Vector2, v2: &Vector2) -> bool {
        let floor_v1 = floor(v1);
        let floor_v2 = floor(v2);
        floor_v1.x == floor_v2.x && floor_v1.y == floor_v2.y
    }

    pub fn eq_round(v1: &Vector2, v2: &Vector2) -> bool {
        let round_v1 = round(v1);
        let round_v2 = round(v2);
        round_v1.x == round_v2.x && round_v1.y == round_v2.y
    }

    pub fn beyond(v: &Vector2, b: &Vector2, dir: &Vector2) -> bool {
        let dist = dist_v(&v, &b);
        (dist.x > 0.0 && dir.x < 0.0) 
            || (dist.y > 0.0 && dir.y < 0.0)
            || (dist.x < 0.0 && dir.x > 0.0)
            || (dist.y < 0.0 && dir.y > 0.0)
            || (dist.x == 0.0 && dist.y == 0.0)
    }

    pub fn copy(v: &Vector2) -> Vector2 {
        Vector2 {
            x: v.x,
            y: v.y,
        }
    }

    /// Assuming minimum is 0.0, 0.0
    pub fn wrap(v: &Vector2, max_x: f32, max_y: f32) -> Vector2 {
        let mut r = copy(v);
        if v.x < 0.0 { r.x = max_x }
        if v.x > max_x { r.x = 0.0 }
        if v.y < 0.0 { r.y = max_y }
        if v.y > max_y { r.y = 0.0 }
        r
    }

    pub fn move_toward(v: &Vector2, t: &Vector2, d: f32) -> Vector2 {
        let mut dir: Vector2 = sub(t, &v);
        let dist: f32 = len(&dir);
        if !is_zero(&dir) {
            dir = norm(&dir);
        }
        if d >= dist {
            return *t;
        }
        return scale(&add(&v, &dir), d);
    }

    pub fn velocity_to(start: &Vector2, end: &Vector2, duration: f32) -> Vector2 {
        let dist_x = end.x - start.x;
        let dist_y = end.y - start.y;
        if duration > 0.0 {
            return Vector2 {
                x: dist_x / duration,
                y: dist_y / duration,
            }
        }
        Vector2 {x: dist_x, y: dist_y}
    }

    pub fn gravitate(obj: &Vector2, star: &Vector2) -> Vector2 {
        let gravity = 6.6743 * 10.0;
        let dx = star.x - obj.x;
        let dy = star.y - obj.y;
        let dist_s = dx * dx + dy * dy;
        if dist_s.sqrt() > 0.5 {
            let force = gravity / dist_s;
            return Vector2 {
                x: dx * force ,
                y: dy * force ,
            }
        }
        zero()
    }
}

