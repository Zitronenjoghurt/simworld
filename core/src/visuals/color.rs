#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RGBA([u8; 4]);

impl RGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub const fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self([
            (hex >> 24) as u8,
            (hex >> 16) as u8,
            (hex >> 8) as u8,
            (hex) as u8,
        ])
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

impl From<u8> for RGBA {
    fn from(v: u8) -> Self {
        Self([v; 4])
    }
}

impl From<u32> for RGBA {
    fn from(v: u32) -> Self {
        Self::from_hex(v)
    }
}

impl From<RGBA> for u32 {
    fn from(v: RGBA) -> Self {
        (v.0[0] as u32) << 24 | (v.0[1] as u32) << 16 | (v.0[2] as u32) << 8 | (v.0[3] as u32)
    }
}
