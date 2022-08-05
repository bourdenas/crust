use crate::{
    components::{Id, Position, RigidBody, Size, Velocity},
    physics::CollisionNode,
};
use sdl2::rect::Point;
use specs::prelude::*;

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    entities: Entities<'a>,

    positions: WriteStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
    sizes: ReadStorage<'a, Size>,
    rigid_bodies: ReadStorage<'a, RigidBody>,
}

pub struct MovementSystem {
    null_id: Id,
}

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem {
            null_id: Id(String::default()),
        }
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;
        let mut dirty = BitSet::new();

        for (entity_a, position_a, velocity_a, size_a, _) in (
            &data.entities,
            &data.positions,
            &mut data.velocities,
            &data.sizes,
            &data.rigid_bodies,
        )
            .join()
        {
            if velocity_a.0 == Point::new(0, 0) {
                continue;
            }
            dirty.add(entity_a.id());

            let lhs = CollisionNode {
                entity_id: entity_a.id(),
                id: &self.null_id,
                position: position_a,
                size: size_a,
            };

            for (entity_b, position_b, size_b, _) in (
                &data.entities,
                &data.positions,
                &data.sizes,
                &data.rigid_bodies,
            )
                .join()
            {
                if entity_a == entity_b {
                    continue;
                }

                let rhs = CollisionNode {
                    entity_id: entity_b.id(),
                    id: &self.null_id,
                    position: position_b,
                    size: size_b,
                };

                while velocity_a.0.x() != 0 || velocity_a.0.y() != 0 {
                    let mut lhs_aabb = lhs.aabb();
                    lhs_aabb.offset(velocity_a.0.x(), velocity_a.0.y());
                    if let None = lhs_aabb & rhs.aabb() {
                        break;
                    }

                    let correction = Point::new(
                        match velocity_a.0.x() {
                            0 => 0,
                            i32::MIN..=-1 => 1,
                            1.. => -1,
                        },
                        match velocity_a.0.y() {
                            0 => 0,
                            i32::MIN..=-1 => 1,
                            1.. => -1,
                        },
                    );
                    velocity_a.0 += correction;
                }
            }
        }

        for (position, velocity, _) in (&mut data.positions, &mut data.velocities, &dirty).join() {
            position.0.offset(velocity.0.x(), velocity.0.y());
            velocity.0 = Point::new(0, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{ScalingVec, Size};
    use sdl2::rect::Rect;

    fn create_world() -> World {
        let mut w = World::new();
        w.register::<Position>();
        w.register::<Velocity>();
        w.register::<Size>();
        w.register::<RigidBody>();
        w
    }

    fn create_node(world: &mut World, position: Point, velocity: Point) -> Entity {
        world
            .create_entity()
            .with(Position(position))
            .with(Velocity(velocity))
            .with(Size {
                bounding_box: Rect::new(0, 0, 32, 32),
                scaling: ScalingVec::default(),
            })
            .with(RigidBody {})
            .build()
    }

    #[test]
    fn no_movement() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn no_obstacles() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(2, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(2, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn no_rigid_bodies() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(2, 0));
        world
            .create_entity()
            .with(Position(Point::new(33, 0)))
            .with(Velocity(Point::new(0, 0)))
            .with(Size {
                bounding_box: Rect::new(0, 0, 32, 32),
                scaling: ScalingVec::default(),
            })
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(2, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_collide_partial_movement() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_node(&mut world, Point::new(33, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(1, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_collide_block_movement() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_node(&mut world, Point::new(32, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_overlap_block_movement() {
        let mut world = create_world();
        let node = create_node(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_node(&mut world, Point::new(5, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(node).unwrap().0, Point::new(0, 0));
    }
}
