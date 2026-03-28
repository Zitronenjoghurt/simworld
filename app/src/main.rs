use simworld_core::pop::PopSpawnEvent;
use simworld_core::world::World;

fn main() {
    let mut world = World::new(200, 200);
    world.events.emit(PopSpawnEvent::new([0.0, 0.0].into()));
    world.tick();
    world.tick();
}
