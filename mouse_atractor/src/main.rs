mod physical_body;
use macroquad::{prelude::*, rand::rand};
use rayon::iter::{IntoParallelRefMutIterator,ParallelIterator};

#[macroquad::main("Mouse Atractor")]
async fn main() {
    //To change number of rectangles/particles change this value
    const rect_count: usize = 256;

    let mut phys_bod: [physical_body::PhysicalBody;rect_count] = [physical_body::PhysicalBody::new_on_loc(
            Vec2::new(
                0.0,
                0.0,
            ),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },);rect_count];

    for i in 0..rect_count {
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
                .map(|rect| {
                    rect.interact(mouse_location);
                })
                .for_each(drop);
            }

        phys_bod
            .par_iter_mut()
            .map(|rect| {
                rect.update();
            })
            .for_each(drop);
            
        //render
        phys_bod.iter()
            .map(|rect|{
                draw_rectangle(
                    rect.body.x,
                    rect.body.y,
                    rect.body.w,
                    rect.body.h,
                    rect.color,
                );

            }).for_each(drop);
        next_frame().await
    }
}
