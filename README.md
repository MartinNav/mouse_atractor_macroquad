# mouse_atractor_macroquad
This is just simple saturday project, that implements 'gravity' in macroquad.
Please do not expect any gameplay 😁
And also I made it multithreaded because concurrency in rust is really easy to do.
## How to use
Mouse is attracting the boxes only if you press **left mouse button**, otherwise they continue in their motion.
With **right mouse button** you can spawn more new rectangles.
## ⚠Don't forget⚠
Don't forget to compile with `cargo run --release` or `cargo build --release`, because debug build does have terrible performance, well it had terrible performance but now it is not that bad, after couple of optimilisations, but still the performance can be improved.

