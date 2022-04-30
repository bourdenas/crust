use crate::trust::AnimationScript;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Animated {
    pub script: AnimationScript,
}
