use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Terrain {
    #[default]
    Grass,
    Dirt,
    Water,
    Stone,
}

impl Terrain {
    pub fn sprite_id(&self) -> SpriteId {
        match self {
            Self::Grass => SpriteId::Terrain(TerrainSpriteId::Grass),
            Self::Dirt => SpriteId::Terrain(TerrainSpriteId::Dirt),
            Self::Water => SpriteId::Terrain(TerrainSpriteId::Water),
            Self::Stone => SpriteId::Terrain(TerrainSpriteId::Stone),
        }
    }
}
