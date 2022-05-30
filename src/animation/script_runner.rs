use super::{Animated, Animator};
use crate::{
    components::AnimationRunningState,
    crust::{Animation, AnimationScript},
};
use std::time::Duration;

#[derive(Default)]
pub struct ScriptRunner {
    animator: Animator,
    index: i32,
    iteration: u32,
    speed: f64,
    state: AnimationRunningState,
}

impl ScriptRunner {
    pub fn new(speed: f64) -> Self {
        ScriptRunner {
            speed,
            state: AnimationRunningState::Init,
            ..Default::default()
        }
    }

    pub fn start(&mut self, animated: &mut Animated, script: &AnimationScript) {
        self.state = AnimationRunningState::Running;
        self.index = match self.reversed() {
            false => 0,
            true => script.animation.len() as i32,
        };

        if self.index < 0 || self.index as usize >= script.animation.len() {
            self.stop(animated);
            return;
        }

        self.animator = Animator::new();
        self.animator
            .start(animated, self.animation(script), self.speed);
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

    fn animation<'a>(&self, script: &'a AnimationScript) -> &'a Animation {
        &script.animation[self.index as usize]
    }

    pub fn progress(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
        script: &AnimationScript,
    ) {
        let time_consumed = self.progress_iteration(time_since_last_frame, animated, script);
        if self.finished() {
            self.iteration += 1;
            if script.repeat == 0 || self.iteration < script.repeat {
                // TODO: emit rewind event
                self.start(animated, script);
                self.progress(time_since_last_frame - time_consumed, animated, script);
            }
        }
    }

    fn progress_iteration(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
        script: &AnimationScript,
    ) -> Duration {
        let time_consumed =
            self.animator
                .progress(time_since_last_frame, animated, self.animation(script));

        if self.animator.finished() {
            // TODO: emit animation script part termination

            self.next_animation(animated, script);
            if !self.finished() {
                return self.progress_iteration(
                    time_since_last_frame - time_consumed,
                    animated,
                    script,
                ) + time_consumed;
            }
        }

        time_consumed
    }

    fn next_animation(&mut self, animated: &mut Animated, script: &AnimationScript) {
        match self.reversed() {
            false => self.index += 1,
            true => self.index -= 1,
        };

        if self.index == -1 || self.index as usize == script.animation.len() {
            self.state = AnimationRunningState::Finished;
            return;
        }

        self.animator = Animator::new();
        self.animator
            .start(animated, self.animation(script), self.speed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        animation::testing::Fixture,
        components::ScalingVec,
        crust::{FrameRangeAnimation, TimerAnimation, Vector, VectorAnimation},
    };
    use sdl2::rect::Point;

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

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(1, 0));
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(100), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(3, 0));
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

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(1, 0));
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(110), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(6, 0));
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

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.position.0, Point::new(0, 0));
        assert_eq!(fixture.sprite.frame_index, 2);

        // Progress that only affects one of the animations.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(20), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(1, 0));
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // Check first frame change happens on time.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(30), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(2, 0));
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(runner.finished(), false);

        // FrameRange finishes but the animator is not `finished` as it repeats.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(110), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(8, 0));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Next FrameRange change wraps to the initial frame.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(50), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(10, 0));
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // FrameRange now finishes and the script with it too.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(18, 0));
        assert_eq!(fixture.sprite.frame_index, 5);
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
        let script = multi_leg_script();

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.position.0, Point::new(0, 0));
        assert_eq!(fixture.sprite.frame_index, 2);

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 0));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // 200msec spent on waiting the second leg. Progress is also able to
        // change legs if enough duration has passed.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(220), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 1));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Run the last leg to end.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(80), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 5));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), true);
    }

    #[test]
    fn multi_leg_script_repeats() {
        let mut fixture = Fixture::new();
        let mut script = multi_leg_script();
        script.repeat = 2;

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.position.0, Point::new(0, 0));
        assert_eq!(fixture.sprite.frame_index, 2);

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(150), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 0));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // 200msec spent on waiting the second leg. Progress is also able to
        // change legs if enough duration has passed.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(220), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 1));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        // Run the last leg to end so script now resets. Since start of all
        // animators is called FrameRange already applies a change.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(80), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(7, 5));
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(runner.finished(), false);

        // Run second iteration to end.
        let mut animated = fixture.animated();
        // TODO: There's actually a bug here. If set to progress 450 msec
        // infinite repeatable translation consumes it all even though its
        // parallel frame range would have stopped that earlier. Need to uplevel
        // the iteration on the Animator level from the Performer.
        runner.progress(Duration::from_millis(150), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(14, 5));
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(runner.finished(), false);

        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(300), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(14, 10));
        assert_eq!(fixture.sprite.frame_index, 5);
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

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), false);
        assert_eq!(runner.index, 0);
        assert_eq!(fixture.position.0, Point::new(0, 0));
        assert_eq!(fixture.sprite.frame_index, 0);
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.0, 1.0));

        // Run first leg to finish.
        let mut animated = fixture.animated();
        runner.progress(Duration::from_millis(10), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(0, 0));
        assert_eq!(fixture.sprite.frame_index, 0);
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.2, 2.0));
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

        let mut runner = ScriptRunner::new(1.0);
        let mut animated = fixture.animated();
        runner.start(&mut animated, &script);
        assert_eq!(runner.finished(), true);
    }
}
