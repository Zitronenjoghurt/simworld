use std::any::{Any, TypeId};
use std::collections::HashMap;

type ErasedVec = Box<dyn Any + Send>;

struct ChannelOps {
    clear: fn(&mut ErasedVec),
    new_empty: fn() -> ErasedVec,
}

fn ops<E: Send + 'static>() -> ChannelOps {
    ChannelOps {
        clear: |v| v.downcast_mut::<Vec<E>>().unwrap().clear(),
        new_empty: || Box::new(Vec::<E>::new()),
    }
}

#[derive(Default)]
pub struct Events {
    current: HashMap<TypeId, ErasedVec>,
    pending: HashMap<TypeId, ErasedVec>,
    ops: HashMap<TypeId, ChannelOps>,
}

pub struct EventReader<'a> {
    current: &'a HashMap<TypeId, ErasedVec>,
}

pub struct EventWriter<'a> {
    pending: &'a mut HashMap<TypeId, ErasedVec>,
    ops: &'a mut HashMap<TypeId, ChannelOps>,
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn split(&'_ mut self) -> (EventReader<'_>, EventWriter<'_>) {
        (
            EventReader {
                current: &self.current,
            },
            EventWriter {
                pending: &mut self.pending,
                ops: &mut self.ops,
            },
        )
    }

    pub fn emit<E: Send + 'static>(&mut self, event: E) {
        do_emit(&mut self.pending, &mut self.ops, event);
    }

    pub fn read<E: Send + 'static>(&self) -> impl Iterator<Item = &E> {
        do_read(&self.current)
    }

    pub fn tick(&mut self) {
        self.swap();
    }

    fn swap(&mut self) {
        for (&id, ops) in &self.ops {
            self.current.entry(id).or_insert_with(|| (ops.new_empty)());
            if let Some(v) = self.current.get_mut(&id) {
                (ops.clear)(v);
            }
        }
        std::mem::swap(&mut self.current, &mut self.pending);
    }
}

impl EventReader<'_> {
    pub fn read<E: Send + 'static>(&self) -> impl Iterator<Item = &E> {
        do_read(self.current)
    }
}

impl EventWriter<'_> {
    pub fn emit<E: Send + 'static>(&mut self, event: E) {
        do_emit(self.pending, self.ops, event);
    }
}

fn do_emit<E: Send + 'static>(
    pending: &mut HashMap<TypeId, ErasedVec>,
    ops: &mut HashMap<TypeId, ChannelOps>,
    event: E,
) {
    let id = TypeId::of::<E>();
    ops.entry(id).or_insert_with(self::ops::<E>);
    pending
        .entry(id)
        .or_insert_with(|| Box::new(Vec::<E>::new()))
        .downcast_mut::<Vec<E>>()
        .unwrap()
        .push(event);
}

fn do_read<E: Send + 'static>(current: &HashMap<TypeId, ErasedVec>) -> impl Iterator<Item = &E> {
    current
        .get(&TypeId::of::<E>())
        .into_iter()
        .flat_map(|v| v.downcast_ref::<Vec<E>>().unwrap().iter())
}
