use crate::{animation::ScriptRunner, crust::AnimationScript};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Animation {
    pub runner: ScriptRunner,
}

impl Animation {
    pub fn new(script: AnimationScript) -> Self {
        Animation {
            runner: ScriptRunner::new(script, 1.0),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AnimationRunningState {
    Init,
    Running,
    Paused,
    Finished,
}

impl Default for AnimationRunningState {
    fn default() -> Self {
        AnimationRunningState::Init
    }
}
