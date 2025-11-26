use minifb::Key;
use totalfb::{FbWindow, FrameBuffer};

fn main() {
    let (w, h) = (400, 300);

    let mut fb = FrameBuffer::new(w, h);
    let mut win = FbWindow::new("totalfb example", w, h);

    while win.is_open() && !win.is_key_down(Key::Escape) {
        fb.clear(0x002244);
        fb.set_pixel(100, 100, 0xFFFFFF);
        win.update(fb.raw());
    }
}
