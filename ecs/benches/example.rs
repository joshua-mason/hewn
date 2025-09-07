use std::time::Duration;

use ecs::{run_update_positions_system, ECS};

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 100_000, 500_000, 1000_000, 1000_000_0])]
fn single_movement(bencher: divan::Bencher, n: u64) {
    bencher
        .with_inputs(|| generate_ecs(n))
        .bench_refs(|ecs: &mut ECS| {
            run_movement_system(ecs, 1);
        });
}

fn generate_ecs(n: u64) -> ECS {
    let mut ecs = ECS::new();
    for _ in 0..n {
        ecs.add_entity((0.0, 0.0).into(), (0.0, 0.0).into());
    }
    ecs
}

fn run_movement_system(ecs: &mut ECS, n: u64) {
    let dt = Duration::from_secs(1);
    for _ in 0..n {
        run_update_positions_system(ecs, dt);
    }
}

use ecs::{update_positions, Object, World};

#[divan::bench(args = [100, 100_000, 500_000, 1000_000, 1000_000_0])]
fn single_movement_nonecs(bencher: divan::Bencher, n: u64) {
    bencher
        .with_inputs(|| generate_world(n))
        .bench_refs(|world: &mut World| {
            run_movement_system_nonecs(world, 1);
        });
}

fn generate_world(n: u64) -> World {
    let mut world = World::new();
    for _ in 0..n {
        world.add_object(Object::new(0.0, 0.0, 0.0, 0.0));
    }
    world
}

fn run_movement_system_nonecs(world: &mut World, n: u64) {
    let dt = Duration::from_secs(1);
    for _ in 0..n {
        update_positions(world, dt);
    }
}
