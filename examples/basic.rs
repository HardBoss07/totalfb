use minifb::Key;
use totalfb::{FbWindow, FrameBuffer, FbMode};

fn main() {
    let (w, h) = (400, 300);

    // Normal 1:1 framebuffer
    let fb = FrameBuffer::with_mode(w, h, FbMode::Normal);

    // Create window with scale=1
    let mut win = FbWindow::new("totalfb basic example", w, h, fb.mode_scale());

    let mut fb = fb; // mutable after window is created

    while win.is_open() && !win.is_key_down(Key::Escape) {
        fb.clear(0x002244);
        fb.set_pixel(100, 100, 0xFFFFFF);
        win.update(&fb);
    }
}
