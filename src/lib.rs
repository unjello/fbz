mod error;
pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub struct WindowOptions {
    /// If the window should become fullscreen (default: false)
    pub fullscreen: bool,
}

mod os;

#[cfg(target_os = "linux")]
use self::os::linux as osImpl;

pub struct Window(osImpl::Window);

impl Window {
    pub fn new(name: &str, width: usize, height: usize, _opts: WindowOptions) -> Result<osImpl::Window> {
        osImpl::Window::new(name, width, height, _opts)
    }

    pub fn should_close(&self) -> bool {
        self.0.should_close()
    }

    pub fn close(&self) {
        self.0.close()
    }

    pub fn update(&self, buffer: &[u32]) -> Result<()> {
        self.0.update(buffer)
    }
}
