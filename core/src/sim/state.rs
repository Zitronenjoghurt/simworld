use crate::sim::config::SimConfig;
use crate::sim::performance::SimPerformance;
use crate::visuals::state::VisualState;

#[derive(Default, Clone)]
pub struct SimState {
    pub config: SimConfig,
    pub performance: SimPerformance,
    pub visuals: VisualState,
}

impl SimState {
    pub fn new() -> Self {
        Self::default()
    }
}
