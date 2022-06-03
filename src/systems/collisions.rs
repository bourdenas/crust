use crate::{
    components::{Collisions, Id, Position, Sprite},
    crust::Action,
    physics::{CollisionChecker, CollisionNode},
    resources::SpriteSheetsManager,
};
use specs::prelude::*;
use std::{collections::HashSet, sync::mpsc::Sender};

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,

    ids: ReadStorage<'a, Id>,
    positions: ReadStorage<'a, Position>,
    sprites: ReadStorage<'a, Sprite>,
    collisions: ReadStorage<'a, Collisions>,
}

pub struct CollisionSystem {
    tx: Sender<Action>,
    overlapping_pairs: HashSet<(u32, u32)>,
}

impl CollisionSystem {
    pub fn new(tx: Sender<Action>) -> Self {
        CollisionSystem {
            tx,
            overlapping_pairs: HashSet::default(),
        }
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // let mut data = data;
        for (entity_a, id_a, position_a, sprite_a, collisions) in (
            &data.entities,
            &data.ids,
            &data.positions,
            &data.sprites,
            &data.collisions,
        )
            .join()
        {
            let sprite_sheet_a = match data.sheets_manager.get(&sprite_a.resource) {
                Some(sheet) => sheet,
                None => continue,
            };

            for (entity_b, id_b, position_b, sprite_b) in
                (&data.entities, &data.ids, &data.positions, &data.sprites).join()
            {
                if entity_a == entity_b {
                    continue;
                }
                CollisionChecker::check_collision(
                    CollisionNode {
                        entity_id: entity_a.id(),
                        id: id_a,
                        position: position_a,
                        sprite: sprite_a,
                        sprite_sheet: sprite_sheet_a,
                    },
                    CollisionNode {
                        entity_id: entity_b.id(),
                        id: id_b,
                        position: position_b,
                        sprite: sprite_b,
                        sprite_sheet: match data.sheets_manager.get(&sprite_b.resource) {
                            Some(sheet) => sheet,
                            None => continue,
                        },
                    },
                    &collisions.on_collision,
                    &self.tx,
                    &mut self.overlapping_pairs,
                )
            }
        }
    }
}
