use crate::world::view::ViewState;

#[derive(Default, Clone)]
pub struct SimState {
    pub view: ViewState,
}

impl SimState {
    pub fn new() -> Self {
        Self::default()
    }
}
