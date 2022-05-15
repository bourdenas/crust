use crate::{animation::ScriptRunner, trust::AnimationScript};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct ScriptState {
    pub script: AnimationScript,

    pub speed: f64,
    pub iteration: u32,
    pub runner: ScriptRunner,
    pub state: AnimationRunningState,
}

impl ScriptState {
    pub fn new(script: AnimationScript) -> Self {
        ScriptState {
            script,
            speed: 1.0,
            iteration: 0,
            runner: ScriptRunner::new(),
            state: AnimationRunningState::Init,
        }
    }
}

#[derive(PartialEq, Debug)]
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
