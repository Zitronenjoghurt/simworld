use crate::sim::state::SimState;
use crate::world::World;

pub struct SimContext {
    pub state_writer: triple_buffer::Input<SimState>,
    pub world: World,
}

impl SimContext {
    pub fn run(mut self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
