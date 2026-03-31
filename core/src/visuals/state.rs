use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;

#[derive(Clone)]
pub struct VisualState {
    sprites: Vec<SpritePos>,
}

impl Default for VisualState {
    fn default() -> Self {
        let sprites = vec![
            SpritePos {
                x: 2.0,
                y: 2.0,
                z: 0.0,
                sprite_id: SpriteId::Terrain(TerrainSpriteId::Water),
            },
            SpritePos {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                sprite_id: SpriteId::Terrain(TerrainSpriteId::Dirt),
            },
            SpritePos {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                sprite_id: SpriteId::Terrain(TerrainSpriteId::Grass),
            },
        ];
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
