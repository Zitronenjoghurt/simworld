use crate::error::CoreResult;
use crate::visuals::palette::Palette;
use crate::visuals::sprite::{Sprite, SpriteId};
use crate::visuals::sprite_sheet::terrain::TerrainSpriteId;
use crate::visuals::sprite_sheet::SpriteSheet;
use std::collections::HashMap;

#[derive(Default)]
pub struct SpriteAtlas {
    sprites: Vec<Sprite>,
    static_map: HashMap<SpriteId, usize>,
}

impl SpriteAtlas {
    pub fn load(palette: &Palette) -> CoreResult<Self> {
        let mut atlas = Self::default();

        let terrain = SpriteSheet::from_png(
            include_bytes!("../../../assets/game/terrain/terrain.png"),
            palette,
        )?;
        atlas.register_static_sprite_sheet(terrain, |i| {
            TerrainSpriteId::from_repr(i).map(SpriteId::Terrain)
        });

        Ok(atlas)
    }

    fn register_static_sprite_sheet(
        &mut self,
        sheet: SpriteSheet,
        id_fn: impl Fn(usize) -> Option<SpriteId>,
    ) {
        for (i, sprite) in sheet.into_sprites().into_iter().enumerate() {
            let Some(id) = id_fn(i) else {
                continue;
            };
            let index = self.sprites.len();
            self.sprites.push(sprite);
            self.static_map.insert(id, index);
        }
    }

    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    pub fn sprites(&self) -> impl Iterator<Item = &Sprite> {
        self.sprites.iter()
    }

    pub fn atlas_index(&self, id: SpriteId) -> Option<usize> {
        self.static_map.get(&id).copied()
    }

    pub fn sprites_per_row(&self) -> usize {
        self.sprites.len().isqrt().max(1)
    }
}
