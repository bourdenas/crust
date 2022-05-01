use crate::trust::FrameRangeAnimation;
use specs::prelude::*;
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct FrameRange {
    pub animation: FrameRangeAnimation,
    pub speed: f64,
    pub step: i32,
    pub run_number: i32,
    pub wait_time: Duration,
    pub state: AnimationState,
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
