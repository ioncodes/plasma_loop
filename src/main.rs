extern crate minifb;

use minifb::{InputCallback, Window, Key, Scale, WindowOptions};
use std::{thread, time};
use std::f32;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const SPEED: usize = 2000;
const RING_DENSITY: u32 = 1; // 1 for none

struct KeyCharCallback;

impl InputCallback for KeyCharCallback {
    fn add_char(&mut self, c: u32) {
        println!("add_char {}", c);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Plasma Loop - Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                     resize: false,
                                     scale: Scale::X2,
                                     ..WindowOptions::default()
                                 })
            .expect("Unable to Open Window");

    window.set_input_callback(Box::new(KeyCharCallback {}));

    let mut counter = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let long = 32.0;
        let short = 16.0;
        let dens = 512.0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel =
                    (dens + (dens * (x as f32 / long).sin()) + dens +
                     (dens * (y as f32 / short).sin()) + dens +
                     (dens * ((x + y) as f32 / long).sin()) + dens +
                     (dens * (((x * x + y * y) as f32 + counter as f32).sqrt() / short).sin())) as // + counter for animation
                    u32 / 2;
                buffer[(y * WIDTH) + x] = pixel ^ RING_DENSITY;
            }
        }
        counter += SPEED;

        window
            .get_keys()
            .map(|keys| for t in keys {
                     match t {
                         _ => (),
                     }
                     let ten_millis = time::Duration::from_millis(10);

                     thread::sleep(ten_millis);
                 });

        window.update_with_buffer(&buffer);
    }
}
