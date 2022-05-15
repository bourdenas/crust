use super::{Animated, Performer};
use crate::{components::AnimationRunningState, trust::Animation};
use sdl2::rect::Point;

#[derive(Default)]
pub struct TranslationPerformer {
    iteration: u32,
}

impl Performer for TranslationPerformer {
    fn start(&mut self, _animated: &mut Animated, _animation: &Animation, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState {
        let translation = animation.translation.as_ref().unwrap();
        if let Some(vec) = &translation.vec {
            animated.position.0 += Point::new(vec.x as i32, vec.y as i32);
        }

        self.iteration += 1;
        match translation.repeat > 0 && self.iteration == translation.repeat {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}

impl TranslationPerformer {
    pub fn new() -> Self {
        TranslationPerformer::default()
    }
}
