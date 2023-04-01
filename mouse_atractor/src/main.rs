mod physical_body;
use macroquad::{prelude::*, rand::rand};

#[macroquad::main("Mouse Atractor")]
async fn main() {
    let mut phys_bod: Vec<physical_body::PhysicalBody> = Vec::with_capacity(16);

    for _ in 0..16 {
        phys_bod.push(physical_body::PhysicalBody::new_on_loc(
            Vec2::new(
                rand::gen_range(50.0, screen_width() - 50.0),
                rand::gen_range(50.0, screen_height() - 50.0),
            ),
            Color {
                r: rand::gen_range(0.5, 1.0),
                g: rand::gen_range(0.5, 1.0),
                b: rand::gen_range(0.5, 1.0),
                a: 1.0,
            },
        ));
    }
    //Main game loop
    loop {
        clear_background(BLACK);
        let mouse_location = mouse_position();


            if is_mouse_button_down(MouseButton::Left){
                phys_bod
                .iter_mut()
                .map(|cube| {
                    cube.interact(mouse_location);
                })
                .collect::<Vec<_>>();
            }

        phys_bod
            .iter_mut()
            .map(|cube| {
                cube.update();
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
