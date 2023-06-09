use macroquad::{
    prelude::{Color, Rect, Vec2},
    time::get_frame_time,
};
#[derive(Copy, Clone)]
pub struct PhysicalBody {
    speed: Vec2,
    pub body: Rect,
    pub color: Color,
}
impl PhysicalBody {
    ///# new_on_loc
    /// This function will create new instance of PB on specified `location` with specified `color`.
    #[inline(always)]
    pub fn new_on_loc(location: Vec2, color: Color) -> Self {
        PhysicalBody {
            speed: Vec2 { x: 0.0, y: 0.0 },
            body: Rect {
                x: location.x,
                y: location.y,
                w: 10.0,
                h: 10.0,
            },
            color: color,
        }
    }
    ///# interact
    /// This function will calculate direction and speed of PB.
    /// *Call it whenever you want the PB to interact with attractor*
    pub fn interact(&mut self, mouse_p: (f32, f32)) {
        let curr_loc = (self.body.x, self.body.y);
        let distance = Vec2::distance(
            Vec2 {
                x: mouse_p.0,
                y: mouse_p.1,
            },
            Vec2 {
                x: curr_loc.0,
                y: curr_loc.1,
            },
        );
        self.speed += Vec2::new(
            (mouse_p.0 - curr_loc.0) / distance,
            (mouse_p.1 - curr_loc.1) / distance,
        );
    }
    ///# update
    /// Update function will apply speed and direction of PB on the PB to change its position.
    /// *Shold be called before each frame is rendered*
    pub fn update(&mut self) {
        let frame_time = get_frame_time();
        self.body.x += self.speed.x * frame_time;
        self.body.y += (self.speed.y + 0.1) * frame_time;
    }
}
