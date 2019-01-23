extern crate x11_dl;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::ptr::{null, null_mut};
use x11_dl::keysym::*;
use x11_dl::xlib;

mod error;
pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub struct WindowOptions {
    /// If the window should become fullscreen (default: false)
    pub fullscreen: bool,
}

pub struct Window {
    handle: u64,
    display: *mut xlib::Display,
    xlib: xlib::Xlib,
    wm_delete_window: c_ulong,
    wm_protocols: c_ulong,
}

/*fn list_pixmap_formats(display: *mut xlib::Display, xlib: &xlib::Xlib) -> &[xlib::XPixmapFormatValues] {
  unsafe {
    let mut formats_count : c_int = 0;
    let formats = (xlib.XListPixmapFormats)(display, &mut formats_count);
    let formats = std::slice::from_raw_parts(formats, formats_count as usize);
    formats
  }
}*/

impl Window {
    pub fn new(name: &str, width: usize, height: usize, _opts: WindowOptions) -> Result<Window> {
        let title = match CString::new(name) {
            Err(_) => {
                println!("Unable to convert {} to c_string", name);
                return Err(Error::WindowCreateFailed(
                    "Unable to set correct name".to_owned(),
                ));
            }
            Ok(n) => n,
        };

        unsafe {
            let xlib = xlib::Xlib::open().unwrap();

            let display = (xlib.XOpenDisplay)(null());
            if display == null_mut() {
                panic!("display");
            }
            let screen = (xlib.XDefaultScreen)(display);
            let depth = (xlib.XDefaultDepth)(display, screen);
            let visual = (xlib.XDefaultVisual)(display, screen);
            //let list = list_pixmap_formats(display, &xlib);
            //let list : Vec<&xlib::XPixmapFormatValues> = list.iter().filter(|x| x.depth == depth).collect();

            let screen_width = (xlib.XDisplayWidth)(display, screen);
            let screen_height = (xlib.XDisplayHeight)(display, screen);

            let default_root_window = (xlib.XDefaultRootWindow)(display);

            let mut window_attributes: xlib::XSetWindowAttributes = mem::uninitialized();
            let window = (xlib.XCreateWindow)(
                display,
                default_root_window,
                (screen_width - width as i32) / 2,
                (screen_height - height as i32) / 2,
                width as u32,
                height as u32,
                0,
                depth,
                xlib::InputOutput as c_uint,
                visual,
                xlib::CWBackPixel | xlib::CWBorderPixel | xlib::CWBackingStore,
                &mut window_attributes,
            );
            (xlib.XStoreName)(display, window, title.as_ptr() as *mut c_char);

            (xlib.XSelectInput)(
                display,
                window,
                xlib::StructureNotifyMask | xlib::KeyReleaseMask,
            );

            // Hook close requests.
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

            let wm_protocols = (xlib.XInternAtom)(display, wm_protocols_str.as_ptr(), xlib::False);
            let wm_delete_window =
                (xlib.XInternAtom)(display, wm_delete_window_str.as_ptr(), xlib::False);

            let mut protocols = [wm_delete_window];

            (xlib.XSetWMProtocols)(
                display,
                window,
                protocols.as_mut_ptr(),
                protocols.len() as c_int,
            );

            (xlib.XMapWindow)(display, window);

            Ok(Window {
                handle: window,
                display,
                xlib,
                wm_delete_window,
                wm_protocols,
            })
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn should_close(&self) -> bool {
        let mut should_we = false;
        unsafe {
            let count = (self.xlib.XPending)(self.display);
            for _ in 0..count {
                let mut event: xlib::XEvent = mem::uninitialized();
                (self.xlib.XNextEvent)(self.display, &mut event);
                match event.get_type() {
                    xlib::ClientMessage => {
                        let xclient = xlib::XClientMessageEvent::from(event);

                        if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                            let protocol = xclient.data.get_long(0) as xlib::Atom;

                            if protocol == self.wm_delete_window {
                                should_we = true;
                            }
                        }
                    }

                    xlib::KeyRelease => {
                        let key = (self.xlib.XLookupKeysym)(&mut event.key, 0);
                        match key as u32 {
                            XK_Escape => should_we = true,
                            _ => (),
                        }
                    }

                    _ => (),
                }
            }
        }

        should_we
    }

    pub fn close(&self) {
        unsafe {
            (self.xlib.XCloseDisplay)(self.display);
        }
    }
}
