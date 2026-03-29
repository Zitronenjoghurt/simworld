use crate::visuals::color::RGBA;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PaletteIndex(u8);

impl PaletteIndex {
    pub fn color_index(&self) -> usize {
        (self.0 & 0x1F) as usize
    }

    pub fn transparent(&self) -> bool {
        self.0 & 0x20 != 0
    }

    pub fn new_transparent() -> Self {
        Self(0x20)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Palette([RGBA; 32]);

impl Default for Palette {
    fn default() -> Self {
        Self::endesga_32()
    }
}

impl Palette {
    pub fn new() -> Self {
        Self([RGBA::default(); 32])
    }

    pub fn color(&self, index: PaletteIndex) -> RGBA {
        if index.transparent() {
            RGBA::transparent()
        } else {
            self.0[index.color_index()]
        }
    }

    /// Will return transparent if the color is not in the palette.
    pub fn index(&self, color: RGBA) -> PaletteIndex {
        self.0
            .iter()
            .enumerate()
            .find(|(_, c)| **c == color)
            .map(|(index, _)| PaletteIndex(index as u8))
            .unwrap_or(PaletteIndex::new_transparent())
    }

    pub fn colors(&self) -> &[RGBA] {
        &self.0
    }

    /// Source: https://lospec.com/palette-list/endesga-32
    pub fn endesga_32() -> Self {
        Palette([
            RGBA::from_hex(0xBE4A2FFF),
            RGBA::from_hex(0xD77643FF),
            RGBA::from_hex(0xEAD4AAFF),
            RGBA::from_hex(0xE4A672FF),
            RGBA::from_hex(0xB86F50FF),
            RGBA::from_hex(0x733E39FF),
            RGBA::from_hex(0x3E2731FF),
            RGBA::from_hex(0xA22633FF),
            RGBA::from_hex(0xE43B44FF),
            RGBA::from_hex(0xF77622FF),
            RGBA::from_hex(0xFEAE34FF),
            RGBA::from_hex(0xFEE761FF),
            RGBA::from_hex(0x63C74DFF),
            RGBA::from_hex(0x3E8948FF),
            RGBA::from_hex(0x265C42FF),
            RGBA::from_hex(0x193C3EFF),
            RGBA::from_hex(0x124E89FF),
            RGBA::from_hex(0x0099DBFF),
            RGBA::from_hex(0x2CE8F5FF),
            RGBA::from_hex(0xFFFFFFFF),
            RGBA::from_hex(0xC0CBDCFF),
            RGBA::from_hex(0x8B9BB4FF),
            RGBA::from_hex(0x5A6988FF),
            RGBA::from_hex(0x3A4466FF),
            RGBA::from_hex(0x262B44FF),
            RGBA::from_hex(0x181425FF),
            RGBA::from_hex(0xFF0044FF),
            RGBA::from_hex(0x68386CFF),
            RGBA::from_hex(0xB55088FF),
            RGBA::from_hex(0xF6757AFF),
            RGBA::from_hex(0xE8B796FF),
            RGBA::from_hex(0xC28569FF),
        ])
    }
}
