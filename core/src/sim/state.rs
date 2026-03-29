use crate::visuals::state::VisualState;

#[derive(Default, Clone)]
pub struct SimState {
    pub visuals: VisualState,
}

impl SimState {
    pub fn new() -> Self {
        Self::default()
    }
}
