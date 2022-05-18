use super::Animated;
use crate::{components::AnimationRunningState, trust::Animation};
use std::time::Duration;

pub trait Performer {
    fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64);
    fn stop(&mut self, animated: &mut Animated);
    fn pause(&mut self, animated: &mut Animated);
    fn resume(&mut self, animated: &mut Animated);

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState;
}

#[derive(Default)]
pub struct PerformerBase<P: Performer> {
    performer: P,

    wait_time: Duration,
    state: AnimationRunningState,
}

impl<P> PerformerBase<P>
where
    P: Performer,
{
    pub fn new(performer: P) -> Self {
        PerformerBase {
            performer,
            wait_time: Duration::ZERO,
            state: AnimationRunningState::Init,
        }
    }

    pub fn finished(&self) -> bool {
        self.state == AnimationRunningState::Finished
    }

    pub fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64) {
        self.performer.start(animated, animation, speed);
        self.state = AnimationRunningState::Running;
    }

    pub fn progress(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
        animation: &Animation,
        animation_delay: Duration,
    ) -> Duration {
        if animation_delay == Duration::ZERO {
            self.performer.execute(animated, animation);
            self.state = AnimationRunningState::Finished;
            return Duration::ZERO;
        }

        self.wait_time += time_since_last_frame;
        while animation_delay <= self.wait_time {
            self.wait_time -= animation_delay;
            if let AnimationRunningState::Finished = self.performer.execute(animated, animation) {
                self.state = AnimationRunningState::Finished;
                return time_since_last_frame - self.wait_time;
            }
        }

        time_since_last_frame
    }
}
