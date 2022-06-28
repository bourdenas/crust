use super::{
    Animated, FrameListPerformer, FrameRangePerformer, Progressor, ProgressorImpl,
    ScalingPerformer, TimerPerformer, TranslationPerformer,
};
use crate::crust::Animation;
use std::time::Duration;

#[derive(Default)]
pub struct Animator {
    animation: Animation,
    progressors: Vec<Box<dyn Progressor + Send + Sync>>,
    finished: bool,
}

impl Animator {
    pub fn new(animation: Animation) -> Self {
        Animator {
            animation,
            progressors: vec![],
            finished: false,
        }
    }

    pub fn start(&mut self, animated: &mut Animated, speed: f64) {
        let animation = self.animation.clone();
        if let Some(translation) = animation.translation {
            let delay = translation.delay as u64;
            self.progressors.push(Box::new(ProgressorImpl::new(
                TranslationPerformer::new(translation),
                Duration::from_millis(delay),
            )));
        }
        if let Some(scaling) = animation.scaling {
            let delay = scaling.delay as u64;
            self.progressors.push(Box::new(ProgressorImpl::new(
                ScalingPerformer::new(scaling),
                Duration::from_millis(delay),
            )));
        }
        if let Some(frame_range) = animation.frame_range {
            let delay = frame_range.delay as u64;
            self.progressors.push(Box::new(ProgressorImpl::new(
                FrameRangePerformer::new(frame_range),
                Duration::from_millis(delay),
            )));
        }
        if let Some(frame_list) = animation.frame_list {
            let delay = frame_list.delay as u64;
            self.progressors.push(Box::new(ProgressorImpl::new(
                FrameListPerformer::new(frame_list),
                Duration::from_millis(delay),
            )));
        }
        if let Some(timer) = animation.timer {
            let delay = timer.delay as u64;
            self.progressors.push(Box::new(ProgressorImpl::new(
                TimerPerformer::new(timer),
                Duration::from_millis(delay),
            )));
        }

        for progressor in &mut self.progressors {
            progressor.start(animated, speed);
        }
    }

    pub fn stop(&mut self, animated: &mut Animated) {
        for progressor in &mut self.progressors {
            progressor.stop(animated);
        }
    }

    pub fn pause(&mut self, animated: &mut Animated) {
        for progressor in &mut self.progressors {
            progressor.pause(animated);
        }
    }

    pub fn resume(&mut self, animated: &mut Animated) {
        for progressor in &mut self.progressors {
            progressor.resume(animated);
        }
    }

    pub fn progress(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
    ) -> Duration {
        let mut time_consumed = Duration::ZERO;
        for progressor in &mut self.progressors {
            let consumed = progressor.progress(time_since_last_frame, animated);
            if consumed > time_consumed {
                time_consumed = consumed;
            }
        }

        let original_len = self.progressors.len();
        self.progressors.retain(|progressor| !progressor.finished());

        if self.progressors.len() < original_len {
            self.finished = self.progressors.is_empty() || !self.animation.wait_all;
        }

        time_consumed
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}
