use crate::components::{AnimationRunningState, Position, TranslationState};
use core::time::Duration;
use sdl2::rect::Point;

pub struct TranslationPerformer<'a> {
    translation: &'a mut TranslationState,
    position: &'a mut Position,
}

impl<'a> TranslationPerformer<'a> {
    pub fn new(translation: &'a mut TranslationState, position: &'a mut Position) -> Self {
        TranslationPerformer {
            translation,
            position,
        }
    }

    pub fn run(&mut self, time_since_last_frame: &Duration) {
        if self.translation.state == AnimationRunningState::Init {
            self.translation.state = AnimationRunningState::Running;
        }
        if self.translation.state == AnimationRunningState::Running {
            self.progress(&*time_since_last_frame);
        }
    }

    fn progress(&mut self, time_since_last_frame: &Duration) -> Duration {
        if self.translation.animation.delay == 0 {
            self.execute();
            self.translation.state = AnimationRunningState::Finished;
            return Duration::ZERO;
        }

        self.translation.wait_time += *time_since_last_frame;
        let animation_delay = Duration::from_millis(self.translation.animation.delay as u64);
        while animation_delay < self.translation.wait_time {
            self.translation.wait_time -= animation_delay;
            if let AnimationRunningState::Finished = self.execute() {
                self.translation.state = AnimationRunningState::Finished;
                return *time_since_last_frame - self.translation.wait_time;
            }
        }

        *time_since_last_frame
    }

    fn execute(&mut self) -> AnimationRunningState {
        if let Some(vec) = &self.translation.animation.vec {
            self.position.0 += Point::new(vec.x as i32, vec.y as i32);
        }
        self.translation.run_number += 1;

        match self.translation.animation.repeat > 0
            && self.translation.run_number == self.translation.animation.repeat
        {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}
