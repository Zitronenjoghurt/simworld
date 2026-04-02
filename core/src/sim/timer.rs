use std::time::{Duration, Instant};

pub struct SimTimer {
    accumulator: Duration,
    last_update: Instant,
}

impl Default for SimTimer {
    fn default() -> Self {
        Self {
            accumulator: Duration::ZERO,
            last_update: Instant::now(),
        }
    }
}

impl SimTimer {
    pub fn accumulate(&mut self) {
        let now = Instant::now();
        self.accumulator += now.duration_since(self.last_update);
        self.last_update = now;
    }

    pub fn cap(&mut self, max: Duration) {
        if self.accumulator > max {
            self.accumulator = max;
        }
    }

    pub fn ready(&mut self, period: Duration) -> bool {
        if self.accumulator >= period {
            self.accumulator -= period;
            true
        } else {
            false
        }
    }

    pub fn remaining(&self, period: Duration) -> Duration {
        period.saturating_sub(self.accumulator)
    }
}
