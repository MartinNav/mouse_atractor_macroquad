mod physical_body;
use macroquad::prelude::*;
#[macroquad::main("Mouse Atractor")]
async fn main() {
    let mut phys_bod: Vec<physical_body::PhysicalBody> = Vec::new();
    phys_bod.push(physical_body::PhysicalBody::new_on_loc(
        Vec2::new(350.0, 430.0),
        WHITE,
    ));

    loop {
        clear_background(BLACK);
        phys_bod
            .iter()
            .map(|cube| {
                draw_rectangle(
                    cube.body.x,
                    cube.body.y,
                    cube.body.w,
                    cube.body.h,
                    cube.color,
                )
            })
            .collect::<Vec<_>>();
        next_frame().await
    }
}
