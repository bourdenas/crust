use crate::{
    components::{Id, Position, RigidBody, Velocity},
    physics::CollisionNode,
    resources::SpriteManager,
};
use sdl2::rect::Point;
use specs::prelude::*;

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    entities: Entities<'a>,
    sprite_manager: ReadExpect<'a, SpriteManager>,

    positions: WriteStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
    sprite_info: ReadStorage<'a, SpriteInfo>,
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

        // First Join captures Sprites only. Background tiles do not have a
        // SpriteInfo component.
        for (entity_a, position_a, velocity_a, _) in (
            &data.entities,
            &data.positions,
            &mut data.velocities,
            &data.sprite_info,
            &data.rigid_bodies,
        )
            .join()
        {
            if lhs_velocity.0 == Point::new(0, 0) {
                continue;
            }
            dirty.add(lhs_entity.id());

            let sprite = data
                .sprite_manager
                .get(&lhs_sprite_info.texture_id)
                .unwrap();

            let lhs = CollisionNode::new(
                lhs_entity.id(),
                &self.null_id,
                lhs_position,
                lhs_size,
                sprite.frames[lhs_sprite_info.frame_index].bitmask.as_ref(),
            );

            // Second Join captures any rigid body including tiles.
            for (entity_b, position_b, _) in
                (&data.entities, &data.positions, &data.rigid_bodies).join()
            {
                if lhs_entity == rhs_entity {
                    continue;
                }

                let mut bitmask = None;
                if let Some(rhs_sprite_info) = rhs_sprite_info {
                    let sprite = data
                        .sprite_manager
                        .get(&rhs_sprite_info.texture_id)
                        .unwrap();
                    bitmask = sprite.frames[rhs_sprite_info.frame_index].bitmask.as_ref();
                }
                let rhs = CollisionNode {
                    entity_id: entity_b.id(),
                    id: &self.null_id,
                    position: position_b,
                };

                let rhs = CollisionNode::new(
                    rhs_entity.id(),
                    &self.null_id,
                    rhs_position,
                    rhs_size,
                    bitmask,
                );

                while lhs_velocity.0.x() != 0 || lhs_velocity.0.y() != 0 {
                    let updated_position = Position(lhs.position.0 + lhs_velocity.0);
                    let lhs = CollisionNode::new(
                        lhs.entity_id,
                        lhs.id,
                        &updated_position,
                        lhs.size,
                        lhs.collision_mask,
                    );
                    if let None = lhs.intersection(&rhs) {
                        break;
                    }

                    let correction = Point::new(
                        match lhs_velocity.0.x() {
                            0 => 0,
                            i32::MIN..=-1 => 1,
                            1.. => -1,
                        },
                        match lhs_velocity.0.y() {
                            0 => 0,
                            i32::MIN..=-1 => 1,
                            1.. => -1,
                        },
                    );
                    lhs_velocity.0 += correction;
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

        let sprite_manager = SpriteManager::mock(vec![
            Sprite {
                texture_id: "spriteA".to_owned(),
                frames: vec![Frame {
                    bounding_box: Rect::new(0, 0, 5, 3),
                    bitmask: Some(vec![0, 5, 8, 9, 10, 11, 12, 13].iter().collect()),
                }],
            },
            Sprite {
                texture_id: "spriteB".to_owned(),
                frames: vec![Frame {
                    bounding_box: Rect::new(0, 0, 5, 3),
                    bitmask: Some(vec![0, 1, 6, 7, 10, 11, 12, 13, 14].iter().collect()),
                }],
            },
        ]);
        w.insert(sprite_manager);

        w
    }

    fn create_sprite(world: &mut World, position: Point, velocity: Point) -> Entity {
        world
            .create_entity()
            .with(Position(Rect::new(position.x(), position.y(), 32, 32)))
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
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn no_obstacles() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(2, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn no_rigid_bodies() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));
        world
            .create_entity()
            .with(Position(Point::new(33, 0)))
            .with(Velocity(Point::new(0, 0)))
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(2, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_collide_partial_movement() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_sprite(&mut world, Point::new(6, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(1, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_collide_block_movement() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_sprite(&mut world, Point::new(5, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_overlap_block_movement() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));
        create_sprite(&mut world, Point::new(3, 0), Point::new(0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(node).unwrap().0, Point::new(0, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_with_collision_masks() {
        // Sprites with collision masks below colliding.
        //  LHS  -  RHS
        // 10000   11000
        // 10011   01100
        // 11110   11111
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(3, 0));
        world
            .create_entity()
            .with(Position(Point::new(5, 0)))
            .with(Size {
                bounding_box: Rect::new(0, 0, 5, 3),
                scaling: ScalingVec::default(),
            })
            .with(SpriteInfo {
                texture_id: "spriteB".to_owned(),
                frame_index: 0,
            })
            .with(RigidBody {})
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(sprite).unwrap().0, Point::new(1, 0));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }
}
