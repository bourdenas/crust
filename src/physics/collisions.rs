use std::collections::HashMap;

use crate::{
    components::{Id, Position, Sprite},
    crust::CollisionAction,
    resources::SpriteSheet,
};

#[derive(Default, Debug)]
pub struct CollisionChecker {
    pub collision_directory: HashMap<String, Vec<CollisionAction>>,
}

impl CollisionChecker {
    pub fn new() -> Self {
        CollisionChecker::default()
    }

    pub fn register_collision(&mut self, collision: CollisionAction) {
        self.collision_directory
            .entry(collision.left_id.clone())
            .or_default()
            .push(collision.clone());
        self.collision_directory
            .entry(collision.right_id.clone())
            .or_default()
            .push(collision);
    }

    pub fn check_collision(
        &self,
        lhs: CollisionNode,
        rhs: CollisionNode,
        collisions: &Vec<CollisionAction>,
    ) {
        for collision in collisions {
            if rhs.id.0 == collision.left_id
                || rhs.id.0 == collision.right_id
                || rhs.sprite.resource == collision.left_id
                || rhs.sprite.resource == collision.right_id
            {
                todo!("check collision");
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
