use crate::error::{CoreError, CoreResult};
use crate::visuals::color::RGBA;
use crate::visuals::palette::{Palette, PaletteIndex};
use crate::visuals::sprite::Sprite;

pub mod terrain;

pub struct SpriteSheet {
    sprites: Vec<Sprite>,
}

impl SpriteSheet {
    pub fn get(&self, index: usize) -> Option<&Sprite> {
        self.sprites.get(index)
    }

    #[cfg(feature = "image")]
    pub fn from_png(bytes: &[u8], palette: &Palette) -> CoreResult<Self> {
        let img = image::load_from_memory(bytes)?.to_rgba8();
        let (w, h) = img.dimensions();

        if w % 8 != 0 || h % 8 != 0 {
            return Err(CoreError::InvalidSpriteSheetSize);
        }

        let cols = w / 8;
        let rows = h / 8;
        let mut sprites = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let mut pixels = [PaletteIndex::new_transparent(); 64];
                for y in 0..8u32 {
                    for x in 0..8u32 {
                        let px = img.get_pixel(col * 8 + x, row * 8 + y);
                        pixels[(y * 8 + x) as usize] = if px[3] == 0 {
                            PaletteIndex::new_transparent()
                        } else {
                            palette.index(RGBA::new(px[0], px[1], px[2], px[3]))
                        };
                    }
                }

                if pixels.iter().all(|p| p.transparent()) {
                    continue;
                }

                sprites.push(Sprite::new(pixels));
            }
        }

        Ok(Self { sprites })
    }
}
