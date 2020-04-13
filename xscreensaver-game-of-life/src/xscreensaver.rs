use std::ffi::CString;
use std::mem::MaybeUninit;
use x11::xlib::{
    Display, Window, XBlackPixelOfScreen, XCreateGC, XCreateSimpleWindow, XDefaultScreenOfDisplay,
    XFillRectangle, XFlush, XGetWindowAttributes, XMapWindow, XOpenDisplay, XRootWindowOfScreen,
    XSetForeground, XWhitePixelOfScreen, XWindowAttributes, GC,
};
use crate::gol::GoL;

static CELL_SIZE: u32 = 2;

#[link(name = "X11")]
extern "C" {}

pub struct ScreensaverSetup {
    dpy: *mut Display,
    root_window_id: Window,
    height: i32,
    width: i32,
    graphics_context: GC,
    black_graphics_context: GC,
}

impl ScreensaverSetup {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let xscreensaver_id_str = std::env::var("XSCREENSAVER_WINDOW")
                .ok()
                .unwrap_or(String::new())
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_start_matches("0x")
                .to_string();
            let xscreensaver_id = Window::from_str_radix(&xscreensaver_id_str, 16).ok();
            let dpy = XOpenDisplay(libc::getenv(CString::new("DISPLAY").unwrap().as_ptr()));

            match xscreensaver_id {
                Some(root_window_id) => {
                    let mut attrs = MaybeUninit::<XWindowAttributes>::uninit();
                    XGetWindowAttributes(dpy, root_window_id, attrs.as_mut_ptr());
                    let attrs2 = attrs.assume_init();
                    let g = XCreateGC(dpy, root_window_id, 0, std::ptr::null_mut());
                    XSetForeground(dpy, g, XWhitePixelOfScreen(XDefaultScreenOfDisplay(dpy)));
                    let g2 = XCreateGC(dpy, root_window_id, 0, std::ptr::null_mut());
                    XSetForeground(dpy, g2, XBlackPixelOfScreen(XDefaultScreenOfDisplay(dpy)));
                    Ok(ScreensaverSetup {
                        dpy,
                        root_window_id,
                        height: attrs2.height,
                        width: attrs2.width,
                        graphics_context: g,
                        black_graphics_context: g2,
                    })
                }
                None => {
                    let height = 800;
                    let width = 1200;
                    let screen = XDefaultScreenOfDisplay(dpy);
                    let win = XCreateSimpleWindow(
                        dpy,
                        XRootWindowOfScreen(screen),
                        0,
                        0,
                        width,
                        height,
                        10,
                        XBlackPixelOfScreen(screen),
                        XBlackPixelOfScreen(screen),
                    );

                    let g = XCreateGC(dpy, win, 0, std::ptr::null_mut());
                    XSetForeground(dpy, g, XWhitePixelOfScreen(XDefaultScreenOfDisplay(dpy)));
                    let g2 = XCreateGC(dpy, win, 0, std::ptr::null_mut());
                    XSetForeground(dpy, g2, XBlackPixelOfScreen(XDefaultScreenOfDisplay(dpy)));
                    XMapWindow(dpy, win);
                    Ok(ScreensaverSetup {
                        dpy,
                        root_window_id: win,
                        height: height as i32,
                        width: width as i32,
                        graphics_context: g,
                        black_graphics_context: g2,
                    })
                }
            }
        }
    }

    pub fn draw_game_of_life(&mut self, gol: &GoL) {
        unsafe {
            // Clearing the window results in screen flicker, so draw black rectangles for the dead
            // cells instead.
            // XClearWindow(self.dpy, self.root_window_id);
            for y in 0..gol.height {
                for x in 0..gol.width {
                    if gol.is_alive(x as i32, y as i32) {
                        XFillRectangle(
                            self.dpy,
                            self.root_window_id,
                            self.graphics_context,
                            (x * CELL_SIZE) as i32,
                            (y * CELL_SIZE) as i32,
                            CELL_SIZE,
                            CELL_SIZE,
                        );
                    } else {
                        XFillRectangle(
                            self.dpy,
                            self.root_window_id,
                            self.black_graphics_context,
                            (x * CELL_SIZE) as i32,
                            (y * CELL_SIZE) as i32,
                            CELL_SIZE,
                            CELL_SIZE,
                        );
                    }
                }
            }
            XFlush(self.dpy);
        }
    }

    pub fn height_in_cells(&self) -> u32 {
        self.height as u32 / CELL_SIZE
    }

    pub fn width_in_cells(&self) -> u32 {
        self.width as u32 / CELL_SIZE
    }
}


