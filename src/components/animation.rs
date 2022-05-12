use crate::trust::{AnimationScript, FrameRangeAnimation, VectorAnimation};
use specs::prelude::*;
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct ScriptState {
    pub script: AnimationScript,

    pub index: usize,
    pub speed: f64,
    pub iteration: u32,
    pub state: AnimationRunningState,
}

impl ScriptState {
    pub fn new(script: AnimationScript) -> Self {
        let speed = 1.0;
        let index = match speed < 0.0 {
            true => script.animation.len() - 1,
            false => 0,
        };

        ScriptState {
            script,
            index,
            speed,
            iteration: 0,
            state: AnimationRunningState::Init,
        }
    }
}

#[derive(Default, Debug)]
pub struct TranslationState {
    pub animation: VectorAnimation,

    pub speed: f64,
    pub run_number: i32,
    pub wait_time: Duration,
    pub state: AnimationRunningState,
}

impl Component for TranslationState {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl TranslationState {
    pub fn new(animation: VectorAnimation) -> Self {
        let speed = 1.0;
        let run_number = match speed < 0.0 {
            true => animation.repeat - 1,
            false => 0,
        };

        TranslationState {
            animation,
            speed,
            run_number,
            wait_time: Duration::ZERO,
            state: AnimationRunningState::Init,
        }
    }
}

#[derive(Default, Debug)]
pub struct FrameRangeState {
    pub animation: FrameRangeAnimation,

    pub speed: f64,
    pub step: i32,
    pub run_number: i32,
    pub wait_time: Duration,
    pub state: AnimationRunningState,
}

impl Component for FrameRangeState {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl FrameRangeState {
    pub fn new(animation: FrameRangeAnimation) -> Self {
        let speed = 1.0;
        let run_number = match speed < 0.0 {
            true => animation.repeat - 1,
            false => 0,
        };
        let step = match animation.start_frame < animation.end_frame {
            true => 1,
            false => 0,
        };

        FrameRangeState {
            animation,
            speed,
            step,
            run_number,
            wait_time: Duration::ZERO,
            state: AnimationRunningState::Init,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AnimationRunningState {
    Init,
    Running,
    Finished,
}

impl Default for AnimationRunningState {
    fn default() -> Self {
        AnimationRunningState::Init
    }
}
