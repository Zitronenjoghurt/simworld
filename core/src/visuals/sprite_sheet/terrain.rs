#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(usize)]
pub enum TerrainSpriteId {
    Grass = 0,
    Dirt = 1,
    Water = 2,
    Stone = 3,
}
