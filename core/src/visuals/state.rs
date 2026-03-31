use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;

#[derive(Default, Clone)]
pub struct VisualState {
    terrain: Vec<SpritePos>,
    terrain_initialized: bool,
    dynamic: Vec<SpritePos>,
}

impl VisualState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn terrain_initialized(&self) -> bool {
        self.terrain_initialized
    }

    pub fn initialize_terrain(&mut self, width: usize, height: usize) {
        self.terrain.resize(
            width * height,
            SpritePos {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                sprite_id: SpriteId::Terrain(TerrainSpriteId::Grass),
            },
        );
        self.terrain_initialized = true;
    }

    pub fn set_terrain(&mut self, index: usize, x: f32, y: f32, z: f32, sprite_id: SpriteId) {
        self.terrain[index] = SpritePos { x, y, z, sprite_id };
    }

    pub fn sprites(&self) -> impl Iterator<Item = &SpritePos> {
        self.terrain.iter().chain(self.dynamic.iter())
    }

    pub fn clear(&mut self) {
        self.dynamic.clear();
    }

    pub fn add_dynamic(&mut self, x: f32, y: f32, z: f32, sprite_id: SpriteId) {
        self.dynamic.push(SpritePos { x, y, z, sprite_id });
    }
}

#[derive(Copy, Clone)]
pub struct SpritePos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sprite_id: SpriteId,
}
