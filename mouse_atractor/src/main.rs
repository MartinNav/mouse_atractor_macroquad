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
        let mouse_location = mouse_position();
        phys_bod
            .iter_mut()
            .map(|cube| {
                cube.update(mouse_location);
                draw_rectangle(
                    cube.body.x,
                    cube.body.y,
                    cube.body.w,
                    cube.body.h,
                    cube.color,
                );
            })
            .collect::<Vec<_>>();
        next_frame().await
    }
}
