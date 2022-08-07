use super::{Animated, Animator};
use crate::{
    components::AnimationRunningState,
    crust::{event, Animation, AnimationEvent, AnimationScript, Vector},
};
use std::time::Duration;

#[derive(Default)]
pub struct ScriptRunner {
    pub script: AnimationScript,
    speed: f64,

    animator: Animator,
    index: i32,
    iteration: u32,
    state: AnimationRunningState,
}

impl ScriptRunner {
    pub fn new(script: AnimationScript, speed: f64) -> Self {
        ScriptRunner {
            script,
            speed,
            state: AnimationRunningState::Init,
            ..Default::default()
        }
    }

    pub fn start(&mut self, animated: &mut Animated) {
        self.state = AnimationRunningState::Running;
        self.index = match self.reversed() {
            false => 0,
            true => self.script.animation.len() as i32,
        };

        if self.index < 0 || self.index as usize >= self.script.animation.len() {
            self.stop(animated);
            return;
        }

        self.animator = Animator::new(self.current_animation().clone());
        self.animator.start(animated, self.speed);
    }

    pub fn stop(&mut self, animated: &mut Animated) {
        self.animator.stop(animated);
        self.state = AnimationRunningState::Finished;
    }

    pub fn pause(&mut self, animated: &mut Animated) {
        self.animator.pause(animated);
    }

    pub fn resume(&mut self, animated: &mut Animated) {
        self.animator.resume(animated);
    }

    pub fn finished(&self) -> bool {
        self.state == AnimationRunningState::Finished
    }

    pub fn state(&self) -> AnimationRunningState {
        self.state
    }

    fn reversed(&self) -> bool {
        self.speed < 0.0
    }

    fn current_animation(&self) -> &Animation {
        &self.script.animation[self.index as usize]
    }

    pub fn progress(&mut self, time_since_last_frame: Duration, animated: &mut Animated) {
        let time_consumed = self.progress_iteration(time_since_last_frame, animated);
        if self.finished() {
            self.iteration += 1;
            if self.script.repeat == 0 || self.iteration < self.script.repeat {
                self.emit_script_rewind(animated);
                self.start(animated);
                self.progress(time_since_last_frame - time_consumed, animated);
            }
        }
    }

    fn progress_iteration(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
    ) -> Duration {
        let time_consumed = self.animator.progress(time_since_last_frame, animated);

        if self.animator.finished() {
            self.emit_animation_done(animated);
            self.progress_animation(animated);
            if !self.finished() {
                return self.progress_iteration(time_since_last_frame - time_consumed, animated)
                    + time_consumed;
            }
        }

        time_consumed
    }

    fn progress_animation(&mut self, animated: &mut Animated) {
        match self.reversed() {
            false => self.index += 1,
            true => self.index -= 1,
        };

        if self.index == -1 || self.index as usize == self.script.animation.len() {
            self.state = AnimationRunningState::Finished;
            return;
        }

        self.animator = Animator::new(self.current_animation().clone());
        self.animator.start(animated, self.speed);
    }

    fn emit_script_rewind(&self, animated: &Animated) {
        if let Some(queue) = animated.queue {
            queue.emit(
                format!("{}_script_rewind", animated.id.0),
                event::Event::AnimationScriptRewind(AnimationEvent {
                    animation_id: self.script.id.clone(),
                    position: Some(Vector {
                        x: animated.position.0.x() as f64,
                        y: animated.position.0.y() as f64,
                        z: 0.0,
                    }),
                    frame_index: animated.sprite_info.frame_index as u32,
                }),
            );
        }
    }

    fn emit_animation_done(&self, animated: &Animated) {
        if let Some(queue) = animated.queue {
            queue.emit(
                format!("{}_segment_done", animated.id.0),
                event::Event::AnimationDone(AnimationEvent {
                    animation_id: self.script.id.clone(),
                    position: Some(Vector {
                        x: animated.position.0.x() as f64,
                        y: animated.position.0.y() as f64,
                        z: 0.0,
                    }),
                    frame_index: animated.sprite_info.frame_index as u32,
                }),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        animation::testing::util::Fixture,
        crust::{FrameRangeAnimation, TimerAnimation, Vector, VectorAnimation},
    };
    use sdl2::rect::{Point, Rect};

    #[test]
    fn move_right_finite() {
        let mut fixture = Fixture::new();

        let script = AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![Animation {
                translation: Some(VectorAnimation {
                    vec: Some(Vector {
                        x: 1.0,
                        ..Default::default()
                    }),
                    delay: 20,
                    repeat: 3,
                }),
                ..Default::default()
            }],
            repeat: 1,
        };

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(1, 0));
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(100), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(3, 0));
        assert_eq!(runner.finished(), true);
    }

    #[test]
    fn move_right_infinite() {
        let mut fixture = Fixture::new();

        let script = AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![Animation {
                translation: Some(VectorAnimation {
                    vec: Some(Vector {
                        x: 1.0,
                        ..Default::default()
                    }),
                    delay: 20,
                    repeat: 0, //< infinite
                }),
                ..Default::default()
            }],
            repeat: 1,
        };

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(1, 0));
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(110), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(6, 0));
        assert_eq!(runner.finished(), false);
    }

    #[test]
    fn combined_animation() {
        let mut fixture = Fixture::new();

        let script = AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![Animation {
                translation: Some(VectorAnimation {
                    vec: Some(Vector {
                        x: 1.0,
                        ..Default::default()
                    }),
                    delay: 20,
                    repeat: 0, //< infinite
                }),
                frame_range: Some(FrameRangeAnimation {
                    start_frame: 2,
                    end_frame: 6,
                    delay: 50,
                    repeat: 2,
                    ..Default::default()
                }),
                ..Default::default()
            }],
            repeat: 1,
        };

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.velocity.0, Point::new(0, 0));
        assert_eq!(fixture.sprite_info.frame_index, 2);

        // Progress that only affects one of the animations.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(1, 0));
        assert_eq!(fixture.sprite_info.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // Check first frame change happens on time.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(30), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(2, 0));
        assert_eq!(fixture.sprite_info.frame_index, 3);
        assert_eq!(runner.finished(), false);

        // FrameRange finishes but the animator is not `finished` as it repeats.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(110), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(8, 0));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Next FrameRange change wraps to the initial frame.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(50), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(10, 0));
        assert_eq!(fixture.sprite_info.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // FrameRange now finishes and the script with it too.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(18, 0));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), true);
    }

    fn multi_leg_script() -> AnimationScript {
        AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![
                Animation {
                    translation: Some(VectorAnimation {
                        vec: Some(Vector {
                            x: 1.0,
                            ..Default::default()
                        }),
                        delay: 20,
                        repeat: 0, //< infinite
                    }),
                    frame_range: Some(FrameRangeAnimation {
                        start_frame: 2,
                        end_frame: 6,
                        delay: 50,
                        repeat: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Animation {
                    timer: Some(TimerAnimation {
                        delay: 200,
                        repeat: 1,
                    }),
                    ..Default::default()
                },
                Animation {
                    translation: Some(VectorAnimation {
                        vec: Some(Vector {
                            y: 1.0,
                            ..Default::default()
                        }),
                        delay: 20,
                        repeat: 0, //< infinite
                    }),
                    timer: Some(TimerAnimation {
                        delay: 100,
                        repeat: 1,
                    }),
                    ..Default::default()
                },
            ],
            repeat: 1,
        }
    }

    #[test]
    fn multi_leg_script_runs_once() {
        let mut fixture = Fixture::new();

        let mut runner = ScriptRunner::new(multi_leg_script(), 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.velocity.0, Point::new(0, 0));
        assert_eq!(fixture.sprite_info.frame_index, 2);

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 0));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // 200msec spent on waiting the second leg. Progress is also able to
        // change legs if enough duration has passed.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(220), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 1));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Run the last leg to end.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(80), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 5));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), true);
    }

    #[test]
    fn multi_leg_script_repeats() {
        let mut fixture = Fixture::new();
        let mut script = multi_leg_script();
        script.repeat = 2;

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.velocity.0, Point::new(0, 0));
        assert_eq!(fixture.sprite_info.frame_index, 2);

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 0));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // 200msec spent on waiting the second leg. Progress is also able to
        // change legs if enough duration has passed.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(220), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 1));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Run the last leg to end so script now resets. Since start of all
        // animators is called FrameRange already applies a change.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(80), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(7, 5));
        assert_eq!(fixture.sprite_info.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // Run second iteration to end.
        let mut animated = fixture.animated();
        // TODO: There's actually a bug here. If set to progress 450 msec
        // infinite repeatable translation consumes it all even though its
        // parallel frame range would have stopped that earlier. Need to uplevel
        // the iteration on the Animator level from the Performer.
        runner.progress(Duration::from_millis(150), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(14, 5));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(300), &mut animated);
        assert_eq!(fixture.velocity.0, Point::new(14, 10));
        assert_eq!(fixture.sprite_info.frame_index, 5);
        assert_eq!(runner.finished(), true);
    }

    #[test]
    fn instant_script() {
        let mut fixture = Fixture::new();

        let script = AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![Animation {
                scaling: Some(VectorAnimation {
                    vec: Some(Vector {
                        x: 1.2,
                        y: 2.0,
                        ..Default::default()
                    }),
                    delay: 0,
                    repeat: 1,
                }),
                ..Default::default()
            }],
            repeat: 1,
        };

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.position.0, Rect::new(0, 0, 32, 32));
        assert_eq!(fixture.velocity.0, Point::new(0, 0));
        assert_eq!(fixture.sprite_info.frame_index, 0);

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(10), &mut animated);
        assert_eq!(fixture.position.0, Rect::new(0, 0, 38, 64));
        assert_eq!(fixture.velocity.0, Point::new(0, 0));
        assert_eq!(fixture.sprite_info.frame_index, 0);
        assert_eq!(runner.finished(), true);
    }

    #[test]
    fn empty_script() {
        let mut fixture = Fixture::new();
        let script = AnimationScript {
            id: "move_right".to_owned(),
            animation: vec![],
            repeat: 0,
        };

        let mut runner = ScriptRunner::new(script, 1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated);
        assert_eq!(runner.finished(), true);
    }
}
