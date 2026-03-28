use crate::map::terrain::Terrain;
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
}
