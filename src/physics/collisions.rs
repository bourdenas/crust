use crate::{
    components::{Id, Position, Sprite},
    crust::{action, event, Action, Box, CollisionAction, CollisionEvent, EmitAction, Event},
    resources::SpriteSheet,
};
use sdl2::rect::Rect;
use std::{collections::HashSet, sync::mpsc::Sender};

#[derive(Debug)]
pub struct CollisionChecker {
    tx: Sender<Action>,
    overlapping_pairs: HashSet<(u32, u32)>,
}

impl CollisionChecker {
    pub fn new(tx: Sender<Action>) -> Self {
        CollisionChecker {
            tx,
            overlapping_pairs: HashSet::default(),
        }
    }

    pub fn check_collision(
        &mut self,
        lhs: CollisionNode,
        rhs: CollisionNode,
        collision_actions: &Vec<CollisionAction>,
    ) {
        for collision in collision_actions {
            if rhs.id.0 == collision.other_id || rhs.sprite.resource == collision.other_id {
                let pair = ordered(lhs.entity_id, rhs.entity_id);

                match lhs.aabb() & rhs.aabb() {
                    Some(intersection) => {
                        if self.overlapping_pairs.contains(&pair) {
                            continue;
                        }

                        self.overlapping_pairs.insert(pair);
                        self.emit_collision(lhs.id.0.clone(), rhs.id.0.clone(), &intersection);

                        for action in &collision.action {
                            self.tx
                                .send(action.clone())
                                .expect("🦀 Action channel closed.");
                        }
                    }
                    None => {
                        if self.overlapping_pairs.contains(&pair) {
                            self.overlapping_pairs.remove(&pair);
                            self.emit_detach(lhs.id.0.clone(), rhs.id.0.clone());
                        }
                    }
                }
            }
        }
    }

    fn emit_collision(&self, lhs_id: String, rhs_id: String, intersection: &Rect) {
        self.tx
            .send(Action {
                action: Some(action::Action::Emit(EmitAction {
                    event: Some(Event {
                        event_id: format!("{}_collide", &lhs_id),
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

    fn emit_detach(&self, lhs_id: String, rhs_id: String) {
        self.tx
            .send(Action {
                action: Some(action::Action::Emit(EmitAction {
                    event: Some(Event {
                        event_id: format!("{}_detach", &lhs_id),
                        event: Some(event::Event::OnDetach(CollisionEvent {
                            lhs_id: lhs_id,
                            rhs_id: rhs_id,
                            intersection: None,
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
