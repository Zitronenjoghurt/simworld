use crate::visuals::state::VisualState;
use crate::world::map::terrain::Terrain;
use grid::Grid;

mod terrain;

pub struct Map {
    terrain: Grid<Terrain>,
    dirty_terrain: Vec<usize>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let terrain = Grid::new(height, width);
        Self {
            terrain,
            dirty_terrain: (0..(height * width)).collect(),
        }
    }

    pub fn draw(&mut self, visuals: &mut VisualState) {
        if !visuals.terrain_initialized() {
            let (height, width) = self.terrain.size();
            visuals.initialize_terrain(width, height);
            self.draw_all(visuals);
        } else {
            self.draw_dirty(visuals)
        }
    }

    pub fn draw_all(&self, visuals: &mut VisualState) {
        let cols = self.terrain.cols();
        for (idx, tile) in self.terrain.iter().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            visuals.set_terrain(
                idx,
                (col * 8) as f32,
                (row * 8) as f32,
                0.0,
                tile.sprite_id(),
            );
        }
    }

    pub fn draw_dirty(&mut self, visuals: &mut VisualState) {
        let cols = self.terrain.cols();
        for i in self.dirty_terrain.drain(..) {
            let row = i / cols;
            let col = i % cols;
            let Some(terrain) = self.terrain.get(row, col) else {
                continue;
            };
            visuals.set_terrain(
                i,
                (col * 8) as f32,
                (row * 8) as f32,
                0.0,
                terrain.sprite_id(),
            );
        }
    }
}
