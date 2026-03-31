use crate::sim::state::SimState;
use crate::world::World;

pub struct SimContext {
    pub state_writer: triple_buffer::Input<SimState>,
    pub world: World,
}

impl SimContext {
    pub fn run(mut self) {
        loop {
            let state = self.state_writer.input_buffer_mut();

            state.performance.update.start();

            state.visuals.clear();
            self.world.tick();
            self.world.draw(&mut state.visuals);

            state.performance.update.stop(30);
            self.state_writer.publish();

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
