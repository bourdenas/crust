use crate::{
    action::ActionQueue,
    components::{Id, Position},
    crust::{event, Box, CollisionAction, CollisionEvent},
};
use sdl2::rect::Rect;
use std::collections::HashSet;

pub struct CollisionChecker {
    queue: ActionQueue,
    overlapping_pairs: HashSet<(u32, u32)>,
}

impl CollisionChecker {
    pub fn new(queue: ActionQueue) -> Self {
        CollisionChecker {
            queue,
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
            if rhs.id.0 == collision.other_id {
                let pair = ordered(lhs.entity_id, rhs.entity_id);

                match lhs.aabb() & rhs.aabb() {
                    Some(intersection) => {
                        if self.overlapping_pairs.contains(&pair) {
                            continue;
                        }

                        self.overlapping_pairs.insert(pair);
                        self.emit_collision(lhs.id.0.clone(), rhs.id.0.clone(), &intersection);

                        for action in &collision.action {
                            self.queue.push(action.clone());
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
        self.queue.emit(
            format!("{}_collide", &lhs_id),
            event::Event::OnCollision(CollisionEvent {
                lhs_id,
                rhs_id,
                intersection: Some(Box {
                    left: intersection.x(),
                    top: intersection.y(),
                    width: intersection.width(),
                    height: intersection.height(),
                }),
            }),
        );
    }

    fn emit_detach(&self, lhs_id: String, rhs_id: String) {
        self.queue.emit(
            format!("{}_detach", &lhs_id),
            event::Event::OnDetach(CollisionEvent {
                lhs_id,
                rhs_id,
                intersection: None,
            }),
        );
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
}

impl<'a> CollisionNode<'a> {
    pub fn aabb(&self) -> Rect {
        self.position.0
    }
}
