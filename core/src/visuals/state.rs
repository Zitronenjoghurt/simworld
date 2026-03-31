use crate::visuals::sprite::SpriteId;

#[derive(Default, Clone)]
pub struct VisualState {
    sprites: Vec<SpritePos>,
}

impl VisualState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sprites(&self) -> &[SpritePos] {
        &self.sprites
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }

    pub fn add(&mut self, x: f32, y: f32, z: f32, sprite_id: SpriteId) {
        self.sprites.push(SpritePos { x, y, z, sprite_id });
    }
}

#[derive(Copy, Clone)]
pub struct SpritePos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sprite_id: SpriteId,
}
