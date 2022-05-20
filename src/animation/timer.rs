use super::{Animated, Performer};
use crate::{components::AnimationRunningState, crust::Animation};

#[derive(Default)]
pub struct TimerPerformer;

impl Performer for TimerPerformer {
    fn start(&mut self, _animated: &mut Animated, _animation: &Animation, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(
        &mut self,
        _animated: &mut Animated,
        _animation: &Animation,
    ) -> AnimationRunningState {
        AnimationRunningState::Finished
    }
}

impl TimerPerformer {
    pub fn new() -> Self {
        TimerPerformer::default()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::trust::TimerAnimation;

//     #[test]
//     fn timer_incomplete() {
//         let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
//         let mut perfomer = TimerPerformer::new(&mut timer);

//         perfomer.run(&Duration::from_millis(500));
//         assert_eq!(timer.state, AnimationRunningState::Running);
//     }

//     #[test]
//     fn timer_finished() {
//         let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
//         let mut perfomer = TimerPerformer::new(&mut timer);

//         perfomer.run(&Duration::from_millis(2000));
//         assert_eq!(timer.state, AnimationRunningState::Finished);
//     }

//     #[test]
//     fn timer_finished_in_multiple_steps() {
//         let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
//         let mut perfomer = TimerPerformer::new(&mut timer);

//         perfomer.run(&Duration::from_millis(500));
//         perfomer.run(&Duration::from_millis(500));
//         perfomer.run(&Duration::from_millis(1000));
//         assert_eq!(timer.state, AnimationRunningState::Finished);
//     }
// }
