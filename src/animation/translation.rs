use crate::components::{AnimationState, Position, Translation};
use core::time::Duration;
use sdl2::rect::Point;

pub struct TranslationPerformer<'a> {
    translation: &'a mut Translation,
    position: &'a mut Position,
}

impl<'a> TranslationPerformer<'a> {
    pub fn new(translation: &'a mut Translation, position: &'a mut Position) -> Self {
        TranslationPerformer {
            translation,
            position,
        }
    }

    pub fn run(&mut self, time_since_last_frame: &Duration) {
        if self.translation.state == AnimationState::Init {
            self.translation.state = AnimationState::Running;
        }
        if self.translation.state == AnimationState::Running {
            self.progress(&*time_since_last_frame);
        }
    }

    fn progress(&mut self, time_since_last_frame: &Duration) -> Duration {
        if self.translation.animation.delay == 0 {
            self.execute();
            self.translation.state = AnimationState::Finished;
            return Duration::ZERO;
        }

        self.translation.wait_time += *time_since_last_frame;
        let animation_delay = Duration::from_millis(self.translation.animation.delay as u64);
        while animation_delay < self.translation.wait_time {
            self.translation.wait_time -= animation_delay;
            if let AnimationState::Finished = self.execute() {
                self.translation.state = AnimationState::Finished;
                return *time_since_last_frame - self.translation.wait_time;
            }
        }

        *time_since_last_frame
    }

    fn execute(&mut self) -> AnimationState {
        if let Some(vec) = &self.translation.animation.vec {
            self.position.0 += Point::new(vec.x as i32, vec.y as i32);
        }
        self.translation.run_number += 1;

        match self.translation.animation.repeat > 0
            && self.translation.run_number == self.translation.animation.repeat
        {
            true => AnimationState::Finished,
            false => AnimationState::Running,
        }
    }
}
