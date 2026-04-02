use crate::sim::config::SimConfig;
use crate::sim::performance::SimPerformance;
use crate::sim::state::SimState;
use crate::sim::timer::SimTimer;
use crate::world::World;
use std::time::Duration;

pub struct SimContext {
    pub state_writer: triple_buffer::Input<SimState>,
    pub config: SimConfig,
    pub performance: SimPerformance,
    pub timer: SimTimer,
    pub world: World,
}

impl SimContext {
    pub fn run(mut self) {
        loop {
            self.performance.update.start();

            let ema_periods = self.config.update_rate as usize * 10;
            let timer_period = Duration::from_secs_f64(1.0 / self.config.update_rate as f64);
            self.timer.accumulate();

            self.timer.cap(timer_period * self.config.max_update_debt);

            while self.timer.ready(timer_period) {
                self.performance.tick.start();
                self.world.tick();
                self.performance.tick.stop(ema_periods);
            }

            self.update_state();

            self.performance.update.stop(ema_periods);

            std::thread::sleep(self.timer.remaining(timer_period));
        }
    }

    fn update_state(&mut self) {
        let state = self.state_writer.input_buffer_mut();
        state.visuals.clear();
        self.world.draw(&mut state.visuals);
        state.config = self.config;
        state.performance = self.performance.clone();
        self.state_writer.publish();
    }
}
