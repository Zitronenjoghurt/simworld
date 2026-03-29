use crate::visuals::palette::PaletteIndex;
use crate::visuals::sprite_sheet::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sprite([PaletteIndex; 64]);

impl Sprite {
    pub fn new(pixels: [PaletteIndex; 64]) -> Self {
        Self(pixels)
    }
}

pub enum SpriteId {
    Terrain(terrain::TerrainSpriteId),
}
