use crate::{animation::ScriptRunner, crust::AnimationScript};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct ScriptState {
    pub script: AnimationScript,
    pub runner: ScriptRunner,
}

impl ScriptState {
    pub fn new(script: AnimationScript) -> Self {
        ScriptState {
            script,
            runner: ScriptRunner::new(1.0),
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
