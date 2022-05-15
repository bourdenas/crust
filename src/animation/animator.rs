use super::{Animated, FrameRangePerformer, Performer, TimerPerformer, TranslationPerformer};
use crate::{components::AnimationRunningState, trust::Animation};
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
        if let Some(_) = &animation.translation {
            self.translation = Some(PerformerBase::new(TranslationPerformer::new()));
            self.translation
                .as_mut()
                .unwrap()
                .start(animated, animation, speed);
        }
        if let Some(_) = &animation.frame_range {
            self.frame_range = Some(PerformerBase::new(FrameRangePerformer::new()));
            self.frame_range
                .as_mut()
                .unwrap()
                .start(animated, animation, speed);
        }
        if let Some(_) = &animation.timer {
            self.timer = Some(PerformerBase::new(TimerPerformer::new()));
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
            let translation = animation.translation.as_ref().unwrap();
            let animation_delay = Duration::from_millis(translation.delay as u64);
            let performer_consumed =
                performer.progress(time_since_last_frame, animated, animation, animation_delay);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.translation = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.frame_range {
            let frame_range = animation.frame_range.as_ref().unwrap();
            let animation_delay = Duration::from_millis(frame_range.delay as u64);
            let performer_consumed =
                performer.progress(time_since_last_frame, animated, animation, animation_delay);
            if performer_consumed > time_consumed {
                time_consumed = performer_consumed;
            }
            if performer.finished() {
                self.frame_range = None;
                maybe_finished = true;
            }
        }
        if let Some(performer) = &mut self.timer {
            let timer = animation.timer.as_ref().unwrap();
            let animation_delay = Duration::from_millis(timer.delay as u64);
            let performer_consumed =
                performer.progress(time_since_last_frame, animated, animation, animation_delay);
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

#[derive(Default)]
struct PerformerBase<P: Performer> {
    performer: P,

    wait_time: Duration,
    state: AnimationRunningState,
}

impl<P> PerformerBase<P>
where
    P: Performer,
{
    fn new(performer: P) -> Self {
        PerformerBase {
            performer,
            wait_time: Duration::ZERO,
            state: AnimationRunningState::Init,
        }
    }

    fn finished(&self) -> bool {
        self.state == AnimationRunningState::Finished
    }

    fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64) {
        self.performer.start(animated, animation, speed);
        self.state = AnimationRunningState::Running;
    }

    fn progress(
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
