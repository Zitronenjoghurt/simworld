use crate::events::Events;
use glam::Vec2;
use slotmap::{new_key_type, SlotMap};

pub struct Pop {
    pub id: PopId,
    pub pos: Vec2,
}

new_key_type! {
    pub struct PopId;
}

#[derive(Default)]
pub struct Pops {
    map: SlotMap<PopId, Pop>,
}

impl Pops {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, events: &mut Events) {
        self.process_events(events);
    }

    fn process_events(&mut self, events: &mut Events) {
        for ev in events.read::<PopSpawnEvent>() {
            let _id = self.map.insert_with_key(|id| Pop { id, pos: ev.pos });
        }
    }
}

pub struct PopSpawnEvent {
    pub pos: Vec2,
}

impl PopSpawnEvent {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
