mod physical_body;
use macroquad::{prelude::*, rand::rand};

#[macroquad::main("Mouse Atractor")]
async fn main() {
    let mut phys_bod: [physical_body::PhysicalBody;16] = [physical_body::PhysicalBody::new_on_loc(
            Vec2::new(
                0.0,
                0.0,
            ),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },);16];

    for i in 0..16 {
        phys_bod[i]=physical_body::PhysicalBody::new_on_loc(
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
        );
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
                .for_each(drop);
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
            .for_each(drop);

        next_frame().await
    }
}
