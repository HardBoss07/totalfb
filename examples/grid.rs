use minifb::Key;
use totalfb::{FbMode, FbWindow, FrameBuffer};

fn main() {
    let (w, h) = (160, 120);

    // Use framebuffer with Grid(3)
    let mut fb = FrameBuffer::with_mode(w, h, FbMode::Grid(3));

    // Window uses logical size * scale
    let mut win = FbWindow::new("Grid Mode Example", w, h, fb.mode_scale());

    let mut t = 0;

    while win.is_open() && !win.is_key_down(Key::Escape) {
        fb.clear(0x001122);

        // Animated white dot
        let x = (t / 2) % w;
        let y = (t / 3) % h;

        fb.set_pixel(x, y, 0xFFFFFF);

        // Draw a small pattern to demonstrate chunky pixels
        for i in 0..10 {
            fb.set_pixel(20 + i, 40, 0xFF0000);
            fb.set_pixel(20, 40 + i, 0x00FF00);
            fb.set_pixel(20 + i, 40 + i, 0x0000FF);
        }

        win.update(&fb);
        t += 1;
    }
}
