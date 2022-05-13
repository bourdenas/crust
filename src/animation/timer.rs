use crate::components::{AnimationRunningState, TimerState};
use core::time::Duration;

pub struct TimerPerformer<'a> {
    timer: &'a mut TimerState,
}

impl<'a> TimerPerformer<'a> {
    pub fn new(timer: &'a mut TimerState) -> Self {
        TimerPerformer { timer }
    }

    pub fn run(&mut self, time_since_last_frame: &Duration) {
        if self.timer.state == AnimationRunningState::Init {
            self.timer.state = AnimationRunningState::Running;
        }
        if self.timer.state == AnimationRunningState::Running {
            self.progress(&*time_since_last_frame);
        }
    }

    fn progress(&mut self, time_since_last_frame: &Duration) -> Duration {
        if self.timer.animation.delay == 0 {
            self.execute();
            self.timer.state = AnimationRunningState::Finished;
            return Duration::ZERO;
        }

        self.timer.wait_time += *time_since_last_frame;
        let animation_delay = Duration::from_millis(self.timer.animation.delay as u64);
        while animation_delay <= self.timer.wait_time {
            self.timer.wait_time -= animation_delay;
            if let AnimationRunningState::Finished = self.execute() {
                self.timer.state = AnimationRunningState::Finished;
                return *time_since_last_frame - self.timer.wait_time;
            }
        }

        *time_since_last_frame
    }

    fn execute(&mut self) -> AnimationRunningState {
        AnimationRunningState::Finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::TimerAnimation;

    #[test]
    fn timer_incomplete() {
        let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
        let mut perfomer = TimerPerformer::new(&mut timer);

        perfomer.run(&Duration::from_millis(500));
        assert_eq!(timer.state, AnimationRunningState::Running);
    }

    #[test]
    fn timer_finished() {
        let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
        let mut perfomer = TimerPerformer::new(&mut timer);

        perfomer.run(&Duration::from_millis(2000));
        assert_eq!(timer.state, AnimationRunningState::Finished);
    }

    #[test]
    fn timer_finished_in_multiple_steps() {
        let mut timer = TimerState::new(TimerAnimation { delay: 2000 });
        let mut perfomer = TimerPerformer::new(&mut timer);

        perfomer.run(&Duration::from_millis(500));
        perfomer.run(&Duration::from_millis(500));
        perfomer.run(&Duration::from_millis(1000));
        assert_eq!(timer.state, AnimationRunningState::Finished);
    }
}
