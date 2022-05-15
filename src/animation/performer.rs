use super::Animated;
use crate::{components::AnimationRunningState, trust::Animation};

pub trait Performer {
    fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64);
    fn stop(&mut self, animated: &mut Animated);
    fn pause(&mut self, animated: &mut Animated);
    fn resume(&mut self, animated: &mut Animated);

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState;
}
