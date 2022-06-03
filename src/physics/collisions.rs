use crate::{
    components::{Id, Position, Sprite},
    crust::{action, event, Action, Box, CollisionAction, CollisionEvent, EmitAction, Event},
    resources::SpriteSheet,
};
use sdl2::rect::Rect;
use std::{collections::HashSet, sync::mpsc::Sender};

#[derive(Default, Debug)]
pub struct CollisionChecker {}

impl CollisionChecker {
    pub fn check_collision(
        lhs: CollisionNode,
        rhs: CollisionNode,
        collision_actions: &Vec<CollisionAction>,
        tx: &Sender<Action>,
        overlapping_pairs: &mut HashSet<(u32, u32)>,
    ) {
        for collision in collision_actions {
            if rhs.id.0 == collision.other_id || rhs.sprite.resource == collision.other_id {
                let pair = ordered(lhs.entity_id, rhs.entity_id);

                match lhs.aabb() & rhs.aabb() {
                    Some(intersection) => {
                        if overlapping_pairs.contains(&pair) {
                            continue;
                        }
                        overlapping_pairs.insert(pair);

                        CollisionChecker::emit_collision(
                            lhs.id.0.clone(),
                            rhs.id.0.clone(),
                            &intersection,
                            tx,
                        );

                        for action in &collision.action {
                            tx.send(action.clone()).expect("🦀 Action channel closed.");
                        }
                    }
                    None => {
                        if overlapping_pairs.contains(&pair) {
                            overlapping_pairs.remove(&pair);
                        }
                    }
                }
            }
        }
    }

    fn emit_collision(lhs_id: String, rhs_id: String, intersection: &Rect, tx: &Sender<Action>) {
        tx.send(Action {
            action: Some(action::Action::Emit(EmitAction {
                event: Some(Event {
                    event_id: format!("{}_collision", &lhs_id),
                    event: Some(event::Event::OnCollision(CollisionEvent {
                        lhs_id: lhs_id,
                        rhs_id: rhs_id,
                        intersection: Some(Box {
                            left: intersection.x(),
                            top: intersection.y(),
                            width: intersection.width(),
                            height: intersection.height(),
                        }),
                    })),
                }),
            })),
        })
        .expect("🦀 Action channel closed.");
    }
}

fn ordered(a: u32, b: u32) -> (u32, u32) {
    match a < b {
        true => (a, b),
        false => (b, a),
    }
}

pub struct CollisionNode<'a> {
    pub entity_id: u32,
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
