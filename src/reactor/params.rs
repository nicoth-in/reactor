#[derive(Clone, Copy)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub fn new(r: u8, g: u8, b:u8, a: u8) -> ColorRGBA {
        ColorRGBA {r: r, g: g, b: b, a: a,}
    }
    pub fn mul(&mut self, c: ColorRGBA) {
        let norm_a = (255 - self.a) / 255;
        let a = self.a + (c.a * norm_a);
        let r = self.r + (c.r * norm_a);
        let g = self.g + (c.g * norm_a);
        let b = self.b + (c.b * norm_a);
        self.a = a; self.r = r; self.g = g; self.b = b;
    }
}
