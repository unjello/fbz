extern crate fbz;
extern crate time;

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

    let start = time::precise_time_ns();
    while !window.should_close() {
        let d = time::precise_time_ns() - start;
        let t: f32 = d as f32 / 1000000000.;
        for y in 0..200 {
            for x in 0..320 {
                let uvx = x as f32 / 320.0;
                let uvy = y as f32 / 200.0;

                let r = 0.5 + 0.5 * (t + uvx).cos();
                let g = 0.5 + 0.5 * (t + uvy + 2.0).cos();
                let b = 0.5 + 0.5 * (t + uvx + 4.0).cos();

                let r = if r < 0.0 { 0 } else { (r * 255.) as u32 };
                let g = if g < 0.0 { 0 } else { (g * 255.) as u32 };
                let b = if b < 0.0 { 0 } else { (b * 255.) as u32 };

                buffer[y * 320 + x] = ((r & 0xff) << 16) | ((g & 0xff) << 8) | (b & 0xff);
            }
        }

        window.update(&buffer).unwrap();
    }

    window.close();
}
