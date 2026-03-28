use std::any::{Any, TypeId};
use std::collections::HashMap;

struct Channel {
    current: Box<dyn Any>,
    pending: Box<dyn Any>,
    swap_fn: fn(&mut Channel),
}

impl Channel {
    fn new<E: 'static>() -> Self {
        Self {
            current: Box::new(Vec::<E>::new()),
            pending: Box::new(Vec::<E>::new()),
            swap_fn: |ch| {
                ch.current.downcast_mut::<Vec<E>>().unwrap().clear();
                std::mem::swap(&mut ch.current, &mut ch.pending);
            },
        }
    }

    fn swap(&mut self) {
        (self.swap_fn)(self);
    }
}

#[derive(Default)]
pub struct Events {
    channels: HashMap<TypeId, Channel>,
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit<E: 'static>(&mut self, event: E) {
        self.channels
            .entry(TypeId::of::<E>())
            .or_insert_with(Channel::new::<E>)
            .pending
            .downcast_mut::<Vec<E>>()
            .unwrap()
            .push(event);
    }

    pub fn read<E: 'static>(&self) -> impl Iterator<Item = &E> {
        self.channels
            .get(&TypeId::of::<E>())
            .into_iter()
            .flat_map(|ch| ch.current.downcast_ref::<Vec<E>>().unwrap().iter())
    }

    pub fn swap(&mut self) {
        for ch in self.channels.values_mut() {
            ch.swap();
        }
    }
}
