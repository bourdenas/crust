use super::{
    performer::PerformerBase, Animated, FrameRangePerformer, TimerPerformer, TranslationPerformer,
};
use crate::trust::Animation;
use std::time::Duration;

#[derive(Default)]
pub struct Animator {
    translation: Option<PerformerBase<TranslationPerformer>>,
    frame_range: Option<PerformerBase<FrameRangePerformer>>,
    timer: Option<PerformerBase<TimerPerformer>>,

    finished: bool,
}

impl Animator {
    pub fn new() -> Self {
        Animator::default()
    }

    pub fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64) {
        if let Some(translation) = &animation.translation {
            self.translation = Some(PerformerBase::new(
                TranslationPerformer::new(),
                Duration::from_millis(translation.delay as u64),
            ));
            self.translation
                .as_mut()
                .unwrap()
                .start(animated, animation, speed);
        }
        if let Some(frame_range) = &animation.frame_range {
            self.frame_range = Some(PerformerBase::new(
                FrameRangePerformer::new(),
                Duration::from_millis(frame_range.delay as u64),
            ));
            self.frame_range
                .as_mut()
                .unwrap()
                .start(animated, animation, speed);
        }
        if let Some(timer) = &animation.timer {
            self.timer = Some(PerformerBase::new(
                TimerPerformer::new(),
                Duration::from_millis(timer.delay as u64),
            ));
            self.timer
                .as_mut()
                .unwrap()
                .start(animated, animation, speed);
        }
    }

    pub fn stop(&mut self, _animated: &mut Animated) {}
    pub fn pause(&mut self, _animated: &mut Animated) {}
    pub fn resume(&mut self, _animated: &mut Animated) {}

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn progress(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
        animation: &Animation,
    ) -> Duration {
        let mut maybe_finished = false;

        let mut time_consumed = Duration::ZERO;
        if let Some(performer) = &mut self.translation {
            let performer_consumed = performer.progress(time_since_last_frame, animated, animation);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.translation = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.frame_range {
            let performer_consumed = performer.progress(time_since_last_frame, animated, animation);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.frame_range = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.timer {
            let performer_consumed = performer.progress(time_since_last_frame, animated, animation);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.timer = None;
                maybe_finished = true;
            }
        }

        if maybe_finished {
            self.finished = match (&self.translation, &self.frame_range, &self.timer) {
                (None, None, None) => true,
                _ if !animation.wait_all => true,
                _ => false,
            };
        }

        time_consumed
    }
}
