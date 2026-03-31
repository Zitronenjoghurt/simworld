use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;

#[derive(Debug, Copy, Clone)]
pub struct Terrain {
    kind: TerrainKind,
}

impl Default for Terrain {
    fn default() -> Self {
        Self::new(TerrainKind::Grass)
    }
}

impl Terrain {
    pub fn new(kind: TerrainKind) -> Self {
        Self { kind }
    }

    pub fn sprite_id(&self) -> SpriteId {
        self.kind.sprite_id()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum TerrainKind {
    #[default]
    Grass,
    Dirt,
    Water,
    Stone,
}

impl TerrainKind {
    pub fn sprite_id(&self) -> SpriteId {
        match self {
            Self::Grass => SpriteId::Terrain(TerrainSpriteId::Grass),
            Self::Dirt => SpriteId::Terrain(TerrainSpriteId::Dirt),
            Self::Water => SpriteId::Terrain(TerrainSpriteId::Water),
            Self::Stone => SpriteId::Terrain(TerrainSpriteId::Stone),
        }
    }
}
