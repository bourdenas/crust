use crate::crust::CollisionAction;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Collisions {
    pub on_collision: Vec<CollisionAction>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct RigidBody;
