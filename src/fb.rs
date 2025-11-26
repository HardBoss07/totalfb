#[derive(Clone, Copy)]
pub enum FbMode {
    Normal,    // 1:1 pixels
    Grid(u32), // scale factor (2, 3, 4…)
}

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub mode: FbMode,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
            mode: FbMode::Normal,
        }
    }

    pub fn with_mode(width: usize, height: usize, mode: FbMode) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
            mode,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.data.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    /// Return scaling factor (1 for Normal, N for Grid(N))
    pub fn mode_scale(&self) -> usize {
        match self.mode {
            FbMode::Normal => 1,
            FbMode::Grid(s) => s as usize,
        }
    }

    /// Returns a zero-copy reference to the raw buffer (1:1)
    pub fn raw(&self) -> &[u32] {
        &self.data
    }

    /// Returns a newly generated scaled buffer depending on the mode.
    /// Normal = clone, Grid(N) = N×N pixel blocks.
    pub fn raw_scaled(&self) -> Vec<u32> {
        match self.mode {
            FbMode::Normal => self.data.clone(),

            FbMode::Grid(scale) => {
                let s = scale as usize;
                let sw = self.width * s;
                let sh = self.height * s;

                let mut out = vec![0u32; sw * sh];

                for y in 0..self.height {
                    for x in 0..self.width {
                        let color = self.data[y * self.width + x];

                        // Fill N×N block
                        for dy in 0..s {
                            for dx in 0..s {
                                let ox = x * s + dx;
                                let oy = y * s + dy;
                                out[oy * sw + ox] = color;
                            }
                        }
                    }
                }

                out
            }
        }
    }
}
