use crate::{
    action::ActionQueue,
    components::{Collisions, Id, Position, Size},
    physics::{CollisionChecker, CollisionNode},
};
use specs::prelude::*;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    entities: Entities<'a>,

    ids: ReadStorage<'a, Id>,
    positions: ReadStorage<'a, Position>,
    sizes: ReadStorage<'a, Size>,
    collisions: ReadStorage<'a, Collisions>,
}

pub struct CollisionSystem {
    checker: CollisionChecker,
}

impl CollisionSystem {
    pub fn new(queue: ActionQueue) -> Self {
        CollisionSystem {
            checker: CollisionChecker::new(queue),
        }
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // let mut data = data;
        for (entity_a, id_a, position_a, size_a, collisions) in (
            &data.entities,
            &data.ids,
            &data.positions,
            &data.sizes,
            &data.collisions,
        )
            .join()
        {
            for (entity_b, id_b, position_b, size_b) in
                (&data.entities, &data.ids, &data.positions, &data.sizes).join()
            {
                if entity_a == entity_b {
                    continue;
                }
                self.checker.check_collision(
                    CollisionNode {
                        entity_id: entity_a.id(),
                        id: id_a,
                        position: position_a,
                        size: size_a,
                    },
                    CollisionNode {
                        entity_id: entity_b.id(),
                        id: id_b,
                        position: position_b,
                        size: size_b,
                    },
                    &collisions.on_collision,
                );
            }
        }
    }
}
