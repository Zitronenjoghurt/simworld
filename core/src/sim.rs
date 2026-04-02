use crate::sim::config::SimConfig;
use crate::sim::state::SimState;
use crate::world::World;

pub mod config;
mod context;
mod performance;
mod state;
mod timer;

pub struct Sim {
    state_reader: triple_buffer::Output<SimState>,
    _thread: Option<std::thread::JoinHandle<()>>,
}

impl Sim {
    pub fn new(config: SimConfig, world: World) -> Self {
        let (sw, sr) = triple_buffer::TripleBuffer::new(&SimState::new()).split();

        let ctx = context::SimContext {
            state_writer: sw,
            config,
            performance: Default::default(),
            timer: Default::default(),
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
