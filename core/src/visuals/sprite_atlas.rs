use crate::visuals::sprite::{Sprite, SpriteId};
use std::collections::HashMap;

pub struct SpriteAtlas {
    sprites: Vec<Sprite>,
    static_map: HashMap<SpriteId, usize>,
}

impl SpriteAtlas {}
