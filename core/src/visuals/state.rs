use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;

#[derive(Clone)]
pub struct VisualState {
    sprites: Vec<SpritePos>,
}

impl Default for VisualState {
    fn default() -> Self {
        let mut sprites = Vec::new();

        for x in 0..=100 {
            for y in 0..=100 {
                let sprite_id = TerrainSpriteId::from_repr((x * y) % 4).unwrap();
                sprites.push(SpritePos {
                    x: (x * 8) as f32,
                    y: (y * 8) as f32,
                    z: 0.0,
                    sprite_id: SpriteId::Terrain(sprite_id),
                });
            }
        }

        Self { sprites }
    }
}

impl VisualState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sprites(&self) -> &[SpritePos] {
        &self.sprites
    }
}

#[derive(Copy, Clone)]
pub struct SpritePos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sprite_id: SpriteId,
}
