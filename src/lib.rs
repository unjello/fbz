extern crate x11;
use x11::xlib;
use std::ptr;

mod error;
pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub struct WindowOptions {
    /// If the window should become fullscreen (default: false)
    pub fullscreen: bool,
}

pub struct Window;

impl Window {
  pub fn new(_name: &str, _width: usize, _height: usize, _opts: WindowOptions) -> Result<Window> {
        unsafe {
          let _s_display =  xlib::XOpenDisplay(ptr::null());
        }

        Ok(Window {})
  }
}