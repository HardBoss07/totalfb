use crate::fb::FrameBuffer;
use minifb::{Key, Window, WindowOptions};

pub struct FbWindow {
    window: Window,
}

impl FbWindow {
    /// Creates a window.  
    /// The window size must match the *scaled* framebuffer size.
    pub fn new(title: &str, logical_w: usize, logical_h: usize, scale: usize) -> Self {
        let w = logical_w * scale;
        let h = logical_h * scale;

        let window =
            Window::new(title, w, h, WindowOptions::default()).expect("Failed to create window");

        Self { window }
    }

    /// Update the window with a framebuffer (automatically uses scaling)
    pub fn update(&mut self, fb: &FrameBuffer) {
        let scale = fb.mode_scale();
        let buf = fb.raw_scaled();

        self.window
            .update_with_buffer(&buf, fb.width * scale, fb.height * scale)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}
