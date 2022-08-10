use crate::{
    action::ActionQueue,
    components::{Collisions, Id, Position, SpriteInfo},
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
        for (lhs_entity, lhs_id, lhs_position, lhs_sprite_info, collisions) in (
            &data.entities,
            &data.ids,
            &data.positions,
            &data.sprite_info,
            &data.collisions,
        )
            .join()
        {
            let lhs_node = CollisionNode {
                entity_id: lhs_entity.id(),
                id: lhs_id,
                position: lhs_position,
                collision_mask: data
                    .sprite_manager
                    .get_collision_mask(&lhs_sprite_info.texture_id, lhs_sprite_info.frame_index),
            };

            for (rhs_entity, rhs_id, rhs_position, rhs_sprite_info) in (
                &data.entities,
                &data.ids,
                &data.positions,
                &data.sprite_info,
            )
                .join()
            {
                if lhs_entity == rhs_entity {
                    continue;
                }

                self.checker.check_collision(
                    &lhs_node,
                    &CollisionNode {
                        entity_id: rhs_entity.id(),
                        id: rhs_id,
                        position: rhs_position,
                        collision_mask: data.sprite_manager.get_collision_mask(
                            &rhs_sprite_info.texture_id,
                            rhs_sprite_info.frame_index,
                        ),
                    },
                    &collisions.on_collision,
                );
            }
        }
    }
}
