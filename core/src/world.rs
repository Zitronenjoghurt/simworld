use crate::visuals::state::VisualState;
use events::Events;
use map::Map;
use pop::Pops;

mod events;
mod map;
pub mod pop;

pub struct World {
    pub events: Events,
    pub map: Map,
    pub pops: Pops,
}

impl World {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            events: Events::new(),
            map: Map::new(height, width),
            pops: Pops::new(),
        }
    }

    pub fn tick(&mut self) {
        self.events.tick();

        self.pops.update(&mut self.events);
    }

    pub fn draw(&mut self, visuals: &mut VisualState) {
        self.map.draw(visuals);
    }
}
