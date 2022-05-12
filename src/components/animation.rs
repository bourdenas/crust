use crate::trust::{Animation, AnimationScript, FrameRangeAnimation, VectorAnimation};
use specs::prelude::*;
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Script {
    pub parts: Vec<Animation>,
    pub index: usize,
    pub repeat: u32,

    pub speed: f64,
    pub iteration: u32,
    pub state: AnimationState,
}

impl Script {
    pub fn new(script: AnimationScript) -> Self {
        let speed = 1.0;
        let index = match speed < 0.0 {
            true => script.animation.len() - 1,
            false => 0,
        };

        Script {
            parts: script.animation,
            index,
            repeat: script.repeat as u32,
            speed,
            iteration: 0,
            state: AnimationState::Init,
        }
    }
}

#[derive(Default, Debug)]
pub struct Translation {
    pub animation: VectorAnimation,
    pub speed: f64,
    pub run_number: i32,
    pub wait_time: Duration,
    pub state: AnimationState,
}

impl Component for Translation {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Translation {
    pub fn new(animation: VectorAnimation) -> Self {
        let speed = 1.0;
        let run_number = match speed < 0.0 {
            true => animation.repeat - 1,
            false => 0,
        };

        Translation {
            animation,
            speed,
            run_number,
            wait_time: Duration::ZERO,
            state: AnimationState::Init,
        }
    }
}

#[derive(Default, Debug)]
pub struct FrameRange {
    pub animation: FrameRangeAnimation,
    pub speed: f64,
    pub step: i32,
    pub run_number: i32,
    pub wait_time: Duration,
    pub state: AnimationState,
}

impl Component for FrameRange {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl FrameRange {
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

        FrameRange {
            animation,
            speed,
            step,
            run_number,
            wait_time: Duration::ZERO,
            state: AnimationState::Init,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AnimationState {
    Init,
    Running,
    Finished,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Init
    }
}
