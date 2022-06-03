use super::{Animated, Performer};
use crate::{components::AnimationRunningState, crust::TimerAnimation};

#[derive(Default)]
pub struct TimerPerformer {
    _timer: TimerAnimation,
}

impl Performer for TimerPerformer {
    fn start(&mut self, _animated: &mut Animated, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, _animated: &mut Animated) -> AnimationRunningState {
        AnimationRunningState::Finished
    }
}

impl TimerPerformer {
    pub fn new(timer: TimerAnimation) -> Self {
        TimerPerformer { _timer: timer }
    }
}
