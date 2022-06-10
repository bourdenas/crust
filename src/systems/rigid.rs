use crate::{
    components::{Id, Position, RigidBody, Sprite, Velocity},
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
    velocities: WriteStorage<'a, Velocity>,
    sprites: ReadStorage<'a, Sprite>,
    rigid_bodies: ReadStorage<'a, RigidBody>,
}

pub struct RigidBodiesSystem;

impl<'a> System<'a> for RigidBodiesSystem {
    type SystemData = RigidBodiesSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;
        let mut movements = vec![];

        for (entity_a, id_a, position_a, velocity_a, sprite_a, _) in (
            &data.entities,
            &data.ids,
            &data.positions,
            &mut data.velocities,
            &data.sprites,
            &data.rigid_bodies,
        )
            .join()
        {
            if velocity_a.0 == Point::new(0, 0) {
                continue;
            }

            let sprite_sheet_a = match data.sheets_manager.get(&sprite_a.resource) {
                Some(sheet) => sheet,
                None => continue,
            };
            let lhs = CollisionNode {
                entity_id: entity_a.id(),
                id: id_a,
                position: position_a,
                sprite: sprite_a,
                sprite_sheet: sprite_sheet_a,
            };
            let mut lhs_aabb = lhs.aabb();

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

                lhs_aabb.offset(velocity_a.0.x(), velocity_a.0.y());
                if let Some(intersection) = lhs_aabb & rhs.aabb() {
                    let x_offset = match lhs_aabb.center().x() < intersection.x() {
                        false => intersection.width() as i32,
                        true => -(intersection.width() as i32),
                    };
                    let y_offset = match lhs_aabb.center().y() < intersection.y() {
                        false => intersection.height() as i32,
                        true => -(intersection.height() as i32),
                    };

                    velocity_a.0 += match x_offset < y_offset {
                        true => Point::new(x_offset, 0),
                        false => Point::new(0, y_offset),
                    };
                }
            }
            movements.push(Movement {
                entity: entity_a,
                offset: velocity_a.0,
            });
            velocity_a.0 = Point::new(0, 0);
        }

        for movement in movements {
            if let Some(position) = data.positions.get_mut(movement.entity) {
                position.0 += movement.offset;
            }
        }
    }
}

struct Movement {
    entity: Entity,
    offset: Point,
}
