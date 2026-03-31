use crate::visuals::state::VisualState;
use crate::world::map::terrain::Terrain;
use grid::Grid;

mod terrain;

pub struct Map {
    pub terrain: Grid<Terrain>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let terrain = Grid::new(height, width);
        Self { terrain }
    }

    pub fn draw(&self, visuals: &mut VisualState) {
        for (y, row) in self.terrain.iter_rows().enumerate() {
            for (x, terrain) in row.enumerate() {
                visuals.add(x as f32, y as f32, 0.0, terrain.sprite_id());
            }
        }
    }
}
