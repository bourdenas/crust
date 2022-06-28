use super::Animated;
use crate::components::AnimationRunningState;
use std::time::Duration;

pub trait Performer {
    fn start(&mut self, animated: &mut Animated, speed: f64);
    fn stop(&mut self, animated: &mut Animated);
    fn pause(&mut self, animated: &mut Animated);
    fn resume(&mut self, animated: &mut Animated);

    fn execute(&mut self, animated: &mut Animated) -> AnimationRunningState;
}

pub trait TimeProgressor {
    fn start(&mut self, animated: &mut Animated, speed: f64);
    fn stop(&mut self, animated: &mut Animated);
    fn pause(&mut self, animated: &mut Animated);
    fn resume(&mut self, animated: &mut Animated);

    fn progress(&mut self, time_since_last_frame: Duration, animated: &mut Animated) -> Duration;
    fn finished(&self) -> bool;
}

#[derive(Default)]
pub struct PerformerBase<P: Performer> {
    performer: P,
    animation_delay: Duration,
    wait_time: Duration,
    state: AnimationRunningState,
}

impl<P> PerformerBase<P>
where
    P: Performer,
{
    pub fn new(performer: P, animation_delay: Duration) -> Self {
        PerformerBase {
            performer,
            animation_delay,
            wait_time: Duration::ZERO,
            state: AnimationRunningState::Init,
        }
    }
}

impl<P> TimeProgressor for PerformerBase<P>
where
    P: Performer,
{
    fn start(&mut self, animated: &mut Animated, speed: f64) {
        self.performer.start(animated, speed);
        self.state = AnimationRunningState::Running;
    }

    fn stop(&mut self, animated: &mut Animated) {
        self.performer.stop(animated);
    }

    fn pause(&mut self, animated: &mut Animated) {
        self.performer.pause(animated);
    }

    fn resume(&mut self, animated: &mut Animated) {
        self.performer.resume(animated);
    }

    fn progress(&mut self, time_since_last_frame: Duration, animated: &mut Animated) -> Duration {
        if self.animation_delay == Duration::ZERO {
            self.performer.execute(animated);
            self.state = AnimationRunningState::Finished;
            return Duration::ZERO;
        }

        self.wait_time += time_since_last_frame;
        while self.animation_delay <= self.wait_time {
            self.wait_time -= self.animation_delay;
            if let AnimationRunningState::Finished = self.performer.execute(animated) {
                self.state = AnimationRunningState::Finished;
                return time_since_last_frame - self.wait_time;
            }
        }

        time_since_last_frame
    }

    fn finished(&self) -> bool {
        self.state == AnimationRunningState::Finished
    }
}
