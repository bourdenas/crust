use crate::{
    components::{Id, Position, Sprite},
    crust::{Action, CollisionAction},
    resources::SpriteSheet,
};
use sdl2::rect::Rect;
use std::sync::mpsc::Sender;

#[derive(Default, Debug)]
pub struct CollisionChecker {}

impl CollisionChecker {
    pub fn check_collision(
        lhs: CollisionNode,
        rhs: CollisionNode,
        collisions: &Vec<CollisionAction>,
        tx: &Sender<Action>,
    ) {
        for collision in collisions {
            if rhs.id.0 == collision.other_id || rhs.sprite.resource == collision.other_id {
                if !lhs.aabb().has_intersection(rhs.aabb()) {
                    continue;
                }

                for action in &collision.action {
                    tx.send(action.clone()).expect("🦀 Action channel closed.");
                }
            }
        }
    }
}

pub struct CollisionNode<'a> {
    pub id: &'a Id,
    pub position: &'a Position,
    pub sprite: &'a Sprite,
    pub sprite_sheet: &'a SpriteSheet,
}

impl<'a> CollisionNode<'a> {
    fn aabb(&self) -> Rect {
        let mut aabb = self.sprite_sheet.bounding_boxes[self.sprite.frame_index];
        aabb.reposition(self.position.0);
        aabb.resize(
            (aabb.width() as f64 * self.sprite.scaling.x) as u32,
            (aabb.height() as f64 * self.sprite.scaling.y) as u32,
        );
        aabb
    }
}
