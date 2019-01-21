extern crate fbz;

use fbz::{Window, WindowOptions};

fn main() {
  let mut window = match Window::new("Test", 320, 200, WindowOptions { fullscreen: false }) {
    Ok(w) => w,
    Err(e) => {
      println!("Unable to create window {}", e);
      return;
    }
  };
}