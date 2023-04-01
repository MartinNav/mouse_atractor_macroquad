use macroquad::prelude::{Color, Rect, Vec2};

pub struct PhysicalBody {
    speed: Vec2,
    pub body: Rect,
    pub color: Color,
}
impl PhysicalBody {
    pub fn new_on_loc(location: Vec2, color: Color) -> Self {
        PhysicalBody {
            speed: Vec2 { x: 0.0, y: 0.0 },
            body: Rect {
                x: location.x,
                y: location.y,
                w: 150.0,
                h: 150.0,
            },
            color: color,
        }
    }
}
