use glam::DVec2;

pub struct PopSpawnEvent {
    pub pos: DVec2,
}

impl PopSpawnEvent {
    pub fn new(pos: DVec2) -> Self {
        Self { pos }
    }
}
