use crate::{
    action::ActionQueue,
    components::{Collisions, Id, Position},
    physics::{CollisionChecker, CollisionNode},
    resources::SpriteManager,
};
use specs::prelude::*;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    entities: Entities<'a>,
    sprite_manager: ReadExpect<'a, SpriteManager>,

    ids: ReadStorage<'a, Id>,
    positions: ReadStorage<'a, Position>,
    sprite_info: ReadStorage<'a, SpriteInfo>,
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
        for (entity_a, id_a, position_a, collisions) in
            (&data.entities, &data.ids, &data.positions, &data.collisions).join()
        {
            for (entity_b, id_b, position_b) in (&data.entities, &data.ids, &data.positions).join()
            {
                if lhs_entity == rhs_entity {
                    continue;
                }

                let sprite = data
                    .sprite_manager
                    .get(&rhs_sprite_info.texture_id)
                    .unwrap();

                self.checker.check_collision(
                    CollisionNode {
                        entity_id: entity_a.id(),
                        id: id_a,
                        position: position_a,
                    },
                    CollisionNode {
                        entity_id: entity_b.id(),
                        id: id_b,
                        position: position_b,
                        sprite.frames[rhs_sprite_info.frame_index].bitmask.as_ref(),
                    },
                    &collisions.on_collision,
                );
            }
        }
    }
}
