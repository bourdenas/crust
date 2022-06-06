use super::{
    performer::PerformerBase, Animated, FrameListPerformer, FrameRangePerformer, ScalingPerformer,
    TimerPerformer, TranslationPerformer,
};
use crate::crust::Animation;
use std::time::Duration;

#[derive(Default)]
pub struct Animator {
    animation: Animation,

    translation: Option<PerformerBase<TranslationPerformer>>,
    scaling: Option<PerformerBase<ScalingPerformer>>,
    frame_range: Option<PerformerBase<FrameRangePerformer>>,
    frame_list: Option<PerformerBase<FrameListPerformer>>,
    timer: Option<PerformerBase<TimerPerformer>>,

    finished: bool,
}

impl Animator {
    pub fn new(animation: Animation) -> Self {
        Animator {
            animation,
            ..Default::default()
        }
    }

    pub fn start(&mut self, animated: &mut Animated, speed: f64) {
        let animation = self.animation.clone();
        if let Some(translation) = animation.translation {
            let delay = translation.delay as u64;
            self.translation = Some(PerformerBase::new(
                TranslationPerformer::new(translation),
                Duration::from_millis(delay),
            ));
            self.translation.as_mut().unwrap().start(animated, speed);
        }
        if let Some(scaling) = animation.scaling {
            let delay = scaling.delay as u64;
            self.scaling = Some(PerformerBase::new(
                ScalingPerformer::new(scaling),
                Duration::from_millis(delay),
            ));
            self.scaling.as_mut().unwrap().start(animated, speed);
        }
        if let Some(frame_range) = animation.frame_range {
            let delay = frame_range.delay as u64;
            self.frame_range = Some(PerformerBase::new(
                FrameRangePerformer::new(frame_range),
                Duration::from_millis(delay),
            ));
            self.frame_range.as_mut().unwrap().start(animated, speed);
        }
        if let Some(frame_list) = animation.frame_list {
            let delay = frame_list.delay as u64;
            self.frame_list = Some(PerformerBase::new(
                FrameListPerformer::new(frame_list),
                Duration::from_millis(delay),
            ));
            self.frame_list.as_mut().unwrap().start(animated, speed);
        }
        if let Some(timer) = animation.timer {
            let delay = timer.delay as u64;
            self.timer = Some(PerformerBase::new(
                TimerPerformer::new(timer),
                Duration::from_millis(delay),
            ));
            self.timer.as_mut().unwrap().start(animated, speed);
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
    ) -> Duration {
        let mut maybe_finished = false;

        let mut time_consumed = Duration::ZERO;
        if let Some(performer) = &mut self.translation {
            let performer_consumed = performer.progress(time_since_last_frame, animated);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.translation = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.scaling {
            let performer_consumed = performer.progress(time_since_last_frame, animated);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.scaling = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.frame_range {
            let performer_consumed = performer.progress(time_since_last_frame, animated);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.frame_range = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.frame_list {
            let performer_consumed = performer.progress(time_since_last_frame, animated);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.frame_list = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.timer {
            let performer_consumed = performer.progress(time_since_last_frame, animated);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.timer = None;
                maybe_finished = true;
            }
        }

        if maybe_finished {
            self.finished = match (
                &self.translation,
                &self.scaling,
                &self.frame_range,
                &self.frame_list,
                &self.timer,
            ) {
                (None, None, None, None, None) => true,
                _ if !self.animation.wait_all => true,
                _ => false,
            };
        }

        time_consumed
    }
}
