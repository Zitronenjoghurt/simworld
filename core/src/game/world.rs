use crate::game::events::Events;
use crate::game::map::Map;
use crate::game::pop::Pops;

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
        self.events.swap();

        self.pops.update(&mut self.events);
    }
}
