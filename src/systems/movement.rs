use crate::{
    components::{Id, Position, RigidBody, SpriteInfo, Velocity},
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

    fn run(&mut self, mut data: Self::SystemData) {
        let mut dirty = BitSet::new();

        // First Join captures Sprites only. Background tiles do not have a
        // SpriteInfo component.
        for (lhs_entity, lhs_position, lhs_velocity, lhs_sprite_info, _) in (
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

            // Second Join captures any rigid body including tiles.
            for (rhs_entity, rhs_position, rhs_sprite_info, _) in (
                &data.entities,
                &data.positions,
                (&data.sprite_info).maybe(),
                &data.rigid_bodies,
            )
                .join()
            {
                if lhs_entity == rhs_entity {
                    continue;
                }

                let rhs = CollisionNode {
                    entity_id: rhs_entity.id(),
                    id: &self.null_id,
                    position: rhs_position,
                    collision_mask: match rhs_sprite_info {
                        Some(sprite_info) => data
                            .sprite_manager
                            .get_collision_mask(&sprite_info.texture_id, sprite_info.frame_index),
                        None => None,
                    },
                };

                while lhs_velocity.0.x() != 0 || lhs_velocity.0.y() != 0 {
                    let mut projected_position = lhs_position.0;
                    projected_position.offset(lhs_velocity.0.x(), lhs_velocity.0.y());

                    let lhs = CollisionNode {
                        entity_id: lhs_entity.id(),
                        id: &self.null_id,
                        position: &Position(projected_position),
                        collision_mask: data.sprite_manager.get_collision_mask(
                            &lhs_sprite_info.texture_id,
                            lhs_sprite_info.frame_index,
                        ),
                    };

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
    use crate::resources::{Frame, Sprite};
    use sdl2::rect::Rect;

    fn create_world() -> World {
        let mut w = World::new();
        w.register::<Position>();
        w.register::<Velocity>();
        w.register::<SpriteInfo>();
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
            .with(Position(Rect::new(position.x(), position.y(), 5, 3)))
            .with(Velocity(velocity))
            .with(SpriteInfo {
                texture_id: "spriteA".to_owned(),
                frame_index: 0,
                bounding_box: Rect::new(0, 0, 5, 3),
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
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(0, 0, 5, 3));
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
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(2, 0, 5, 3));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn no_rigid_bodies() {
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(2, 0));
        world
            .create_entity()
            .with(Position(Rect::new(5, 0, 5, 3)))
            .with(Velocity(Point::new(0, 0)))
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(2, 0, 5, 3));
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
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(1, 0, 5, 3));
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
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(0, 0, 5, 3));
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
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(0, 0, 5, 3));
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
            .with(Position(Rect::new(5, 0, 5, 3)))
            .with(SpriteInfo {
                texture_id: "spriteB".to_owned(),
                frame_index: 0,
                bounding_box: Rect::new(0, 0, 5, 3),
            })
            .with(RigidBody {})
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(1, 0, 5, 3));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }

    #[test]
    fn rigid_bodies_with_only_one_collison_mask() {
        // Sprites with collision masks below colliding.
        //  LHS  -  RHS
        // 10000   11111
        // 10011   11111
        // 11110   11111
        let mut world = create_world();
        let sprite = create_sprite(&mut world, Point::new(0, 0), Point::new(3, 0));
        world
            .create_entity()
            .with(Position(Rect::new(6, 2, 5, 3)))
            // NOTE: Lack of SpriteInfo data results to no collision mask.
            .with(RigidBody {})
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem::new(), "move", &[])
            .build();
        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        assert_eq!(positions.get(sprite).unwrap().0, Rect::new(2, 0, 5, 3));
        let velocities = world.read_storage::<Velocity>();
        assert_eq!(velocities.get(sprite).unwrap().0, Point::new(0, 0));
    }
}
