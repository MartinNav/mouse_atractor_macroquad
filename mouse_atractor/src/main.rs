use macroquad::prelude::*;
#[macroquad::main("Mouse Atractor")]
async fn main() {
    //println!("Hello, world!");
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
