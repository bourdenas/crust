use crate::{
    action::ActionQueue,
    components::{Id, Position},
    crust::{event, Box, CollisionAction, CollisionEvent},
};
use sdl2::rect::Rect;
use specs::BitSet;
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
        lhs: &CollisionNode,
        rhs: &CollisionNode,
        collision_actions: &Vec<CollisionAction>,
    ) {
        for collision in collision_actions {
            if rhs.id.0 == collision.other_id {
                let pair = ordered(lhs.entity_id, rhs.entity_id);

                match lhs.intersection(rhs) {
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
                    left: intersection.left(),
                    top: intersection.top(),
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
    pub collision_mask: Option<&'a BitSet>,
}

impl<'a> CollisionNode<'a> {
    pub fn aabb(&self) -> Rect {
        self.position.0
    }

    pub fn intersection(&self, other: &CollisionNode) -> Option<Rect> {
        match self.aabb() & other.aabb() {
            Some(intersection) => self.pixel_collision(other, intersection),
            None => None,
        }
    }

    fn pixel_collision(&self, other: &CollisionNode, intersection: Rect) -> Option<Rect> {
        // TODO: Returned intersection is not the collision intersection but the
        // bounding box intersection, which is not necessarily the same.
        if self.collision_mask == None && other.collision_mask == None {
            return Some(intersection);
        } else if self.collision_mask == None {
            return Self::single_mask_intersection(
                &other.aabb(),
                other.collision_mask.unwrap(),
                intersection,
            );
        } else if other.collision_mask == None {
            return Self::single_mask_intersection(
                &self.aabb(),
                self.collision_mask.unwrap(),
                intersection,
            );
        }

        let lhs_collision_mask = self.collision_mask.unwrap();
        let rhs_collision_mask = other.collision_mask.unwrap();

        let lhs_rel_left = (intersection.left() - self.aabb().left()) as u32;
        let lhs_rel_top = (intersection.top() - self.aabb().top()) as u32;
        let rhs_rel_left = (intersection.left() - other.aabb().left()) as u32;
        let rhs_rel_top = (intersection.top() - other.aabb().top()) as u32;

        for y in 0..intersection.height() as u32 {
            for x in 0..intersection.width() as u32 {
                let lhs_index = (lhs_rel_top + y) * self.aabb().width() + (lhs_rel_left + x);
                let rhs_index = (rhs_rel_top + y) * other.aabb().width() + (rhs_rel_left + x);

                if lhs_collision_mask.contains(lhs_index) && rhs_collision_mask.contains(rhs_index)
                {
                    return Some(intersection);
                }
            }
        }
        None
    }

    /// Checks collision with a single collision mask assuming that the other
    /// sprite fills its bounding box.
    ///
    /// Note: It would be simpler if a fully set BitSet could be created fast,
    /// but this is not supported by its API, so a bit of code duplication is
    /// better.
    fn single_mask_intersection(
        aabb: &Rect,
        collision_mask: &BitSet,
        intersection: Rect,
    ) -> Option<Rect> {
        let rel_left = (intersection.left() - aabb.left()) as u32;
        let rel_top = (intersection.top() - aabb.top()) as u32;

        for y in 0..intersection.height() as u32 {
            for x in 0..intersection.width() as u32 {
                let index = (rel_top + y) * aabb.width() + (rel_left + x);

                if collision_mask.contains(index) {
                    return Some(intersection);
                }
            }
        }
        None
    }
}
