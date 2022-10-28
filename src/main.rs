use notan::draw::*;
use notan::prelude::*;
use specs::{Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, AppState)]
struct Renderer {}

impl Component for Renderer {
    type Storage = VecStorage<Self>;
}

struct ReadPositions;

impl<'a> System<'a> for ReadPositions {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Renderer>();

    let billson = world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Renderer {})
        .build();

    let pollock = world
        .create_entity()
        .with(Position { x: 5.0, y: 2.0 })
        .with(Renderer {})
        .build();

    let mut read_positions = ReadPositions;
    read_positions.run_now(&world);
    world.maintain();
}
