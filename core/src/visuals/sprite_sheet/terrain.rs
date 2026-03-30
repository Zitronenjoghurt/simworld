use strum_macros::FromRepr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, FromRepr)]
#[repr(usize)]
pub enum TerrainSpriteId {
    Grass = 0,
    Dirt = 1,
    Water = 2,
    Stone = 3,
}
