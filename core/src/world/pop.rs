use crate::world::events::Events;
use crate::world::pop::events::PopSpawnEvent;
use glam::DVec2;
use slotmap::{new_key_type, SlotMap};

mod events;

new_key_type! {
    pub struct PopId;
}

pub struct Pop {
    pub id: PopId,
    pub pos: DVec2,
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
        let (reader, _) = events.split();

        for ev in reader.read::<PopSpawnEvent>() {
            let _id = self.map.insert_with_key(|id| Pop { id, pos: ev.pos });
        }
    }
}
