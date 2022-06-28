use super::INDEX;
use crate::{components, crust::CollisionAction};
use specs::prelude::*;

pub struct Collisions;

impl Collisions {
    pub fn on_collision(collision_action: CollisionAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &*index.borrow() {
                entity_id = index.find_entity(&collision_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);

            let mut collisions = world.write_storage::<components::Collisions>();
            match collisions.get_mut(entity) {
                Some(collisions) => {
                    collisions.on_collision.push(collision_action);
                }
                None => {
                    if let Err(e) = collisions.insert(
                        entity,
                        components::Collisions {
                            on_collision: vec![collision_action],
                        },
                    ) {
                        eprintln!("on_collision(): {}", e);
                    }
                }
            }
        }
    }
}
