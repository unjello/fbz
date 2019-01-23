extern crate fbz;

use fbz::{Window, WindowOptions};

fn main() {
    let mut buffer: Vec<u32> = vec![0; 320 * 200];
    let window = match Window::new("Test", 320, 200, WindowOptions { fullscreen: false }) {
        Ok(w) => w,
        Err(e) => {
            println!("Unable to create window {}", e);
            return;
        }
    };

    while !window.should_close() {
        for i in buffer.iter_mut() {
            *i =0xffff000;
        }

        window.update(&buffer).unwrap();
    }

    window.close();
}
