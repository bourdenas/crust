use crate::{
    components::{Id, Position, Sprite},
    physics::{CollisionChecker, CollisionNode},
    resources::SpriteSheetsManager,
};
use specs::prelude::*;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    collision_checker: ReadExpect<'a, CollisionChecker>,
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,

    ids: ReadStorage<'a, Id>,
    positions: ReadStorage<'a, Position>,
    sprites: ReadStorage<'a, Sprite>,
}

#[derive(Default)]
pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // let mut data = data;

        for (entity_a, id_a, position_a, sprite_a) in
            (&data.entities, &data.ids, &data.positions, &data.sprites).join()
        {
            let collisions = data.collision_checker.collision_directory.get(&id_a.0);
            if let Some(collisions) = collisions {
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
                    data.collision_checker.check_collision(
                        CollisionNode {
                            id: id_a,
                            position: position_a,
                            sprite: sprite_a,
                            sprite_sheet: sprite_sheet_a,
                        },
                        CollisionNode {
                            id: id_b,
                            position: position_b,
                            sprite: sprite_b,
                            sprite_sheet: match data.sheets_manager.get(&sprite_b.resource) {
                                Some(sheet) => sheet,
                                None => continue,
                            },
                        },
                        collisions,
                    )
                }
            }
        }
    }
}
