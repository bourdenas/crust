use crate::{
    components::{Dirty, Id, Position, RigidBody, Sprite},
    physics::CollisionNode,
    resources::SpriteSheetsManager,
};
use sdl2::rect::Point;
use specs::prelude::*;

#[derive(SystemData)]
pub struct RigidBodiesSystemData<'a> {
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,

    ids: ReadStorage<'a, Id>,
    positions: WriteStorage<'a, Position>,
    sprites: ReadStorage<'a, Sprite>,
    rigid_bodies: ReadStorage<'a, RigidBody>,
    dirty: ReadStorage<'a, Dirty>,
}

pub struct RigidBodiesSystem;

impl<'a> System<'a> for RigidBodiesSystem {
    type SystemData = RigidBodiesSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;
        let mut adjustments = vec![];

        for (entity_a, id_a, position_a, sprite_a, _) in (
            &data.entities,
            &data.ids,
            &data.positions,
            &data.sprites,
            &data.rigid_bodies,
            // &data.dirty,
        )
            .join()
        {
            let sprite_sheet_a = match data.sheets_manager.get(&sprite_a.resource) {
                Some(sheet) => sheet,
                None => continue,
            };
            let mut adjust = Point::new(0, 0);

            for (entity_b, id_b, position_b, sprite_b, _) in (
                &data.entities,
                &data.ids,
                &data.positions,
                &data.sprites,
                &data.rigid_bodies,
            )
                .join()
            {
                if entity_a == entity_b {
                    continue;
                }
                let lhs = CollisionNode {
                    entity_id: entity_a.id(),
                    id: id_a,
                    position: position_a,
                    sprite: sprite_a,
                    sprite_sheet: sprite_sheet_a,
                };
                let rhs = CollisionNode {
                    entity_id: entity_b.id(),
                    id: id_b,
                    position: position_b,
                    sprite: sprite_b,
                    sprite_sheet: match data.sheets_manager.get(&sprite_b.resource) {
                        Some(sheet) => sheet,
                        None => continue,
                    },
                };

                let lhs_aabb = lhs.aabb();
                if let Some(intersection) = lhs_aabb & rhs.aabb() {
                    let x_offset = match lhs_aabb.center().x() < intersection.x() {
                        false => intersection.width() as i32,
                        true => -(intersection.width() as i32),
                    };
                    let y_offset = match lhs_aabb.center().y() < intersection.y() {
                        false => intersection.height() as i32,
                        true => -(intersection.height() as i32),
                    };

                    adjust += match x_offset < y_offset {
                        true => Point::new(x_offset, 0),
                        false => Point::new(0, y_offset),
                    };
                    adjustments.push(Adjustment {
                        entity: entity_a,
                        offset: adjust,
                    })
                }
            }
        }

        for adjustment in adjustments {
            if let Some(position) = data.positions.get_mut(adjustment.entity) {
                position.0 += adjustment.offset;
            }
        }
    }
}

struct Adjustment {
    entity: Entity,
    offset: Point,
}
