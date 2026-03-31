use crate::sim::state::SimState;
use crate::world::World;

mod context;
mod performance;
mod state;

pub struct Sim {
    state_reader: triple_buffer::Output<SimState>,
    _thread: Option<std::thread::JoinHandle<()>>,
}

impl Sim {
    pub fn new(world: World) -> Self {
        let (sw, sr) = triple_buffer::TripleBuffer::new(&SimState::new()).split();

        let ctx = context::SimContext {
            state_writer: sw,
            world,
        };
        let _thread = std::thread::spawn(move || ctx.run());

        Self {
            state_reader: sr,
            _thread: None,
        }
    }

    pub fn latest_state(&mut self) -> &SimState {
        self.state_reader.read()
    }
}
