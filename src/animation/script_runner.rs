use super::{Animated, Animator};
use crate::trust::{Animation, AnimationScript};
use std::time::Duration;

#[derive(Default)]
pub struct ScriptRunner {
    animator: Animator,
    index: i32,
    speed: f64,
    finished: bool,
}

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner::default()
    }

    pub fn start(&mut self, animated: &mut Animated, script: &AnimationScript, speed: f64) {
        self.finished = false;
        self.speed = speed;
        self.index = match self.reversed() {
            false => 0,
            true => script.animation.len() as i32,
        };

        if self.index < 0 || self.index as usize >= script.animation.len() {
            self.stop(animated);
        }

        self.animator = Animator::new();
        self.animator.start(animated, self.animation(script), speed);
    }

    pub fn stop(&mut self, animated: &mut Animated) {
        self.animator.stop(animated);
        self.finished = true;
    }

    pub fn pause(&mut self, animated: &mut Animated) {
        self.animator.pause(animated);
    }

    pub fn resume(&mut self, animated: &mut Animated) {
        self.animator.resume(animated);
    }

    pub fn finished(&self) -> bool {
        self.finished
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
    ) -> Duration {
        let time_consumed =
            self.animator
                .progress(time_since_last_frame, animated, self.animation(script));

        if self.animator.finished() {
            // TODO: emit animation script part termination

            self.next_animation(animated, script);
            if !self.finished {
                return self.progress(time_since_last_frame - time_consumed, animated, script)
                    + time_consumed;
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
            self.finished = true;
            return;
        }

        self.animator = Animator::new();
        self.animator
            .start(animated, self.animation(script), self.speed);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{Position, Sprite},
        resources::SpriteSheet,
        trust::{Vector, VectorAnimation},
    };

    use super::*;
    use sdl2::rect::{Point, Rect};
    use specs::prelude::*;

    struct Fixture {
        world: World,
        position: Position,
        sprite: Sprite,
        sheet: SpriteSheet,
    }

    impl Fixture {
        fn new() -> Self {
            Fixture {
                world: World::new(),
                position: Position(Point::new(0, 0)),
                sprite: Sprite {
                    resource: "foo".to_owned(),
                    frame_index: 0,
                },
                sheet: SpriteSheet {
                    resource: "foo".to_owned(),
                    bounding_boxes: vec![Rect::new(0, 0, 32, 32)],
                },
            }
        }

        fn animated(&mut self) -> Animated {
            Animated::new(
                self.world.create_entity().build(),
                &mut self.position,
                &mut self.sprite,
                &self.sheet,
            )
        }
    }

    #[test]
    fn move_right() {
        let mut fixture = Fixture::new();
        let mut animated = fixture.animated();

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

        let mut runner = ScriptRunner::new();
        runner.start(&mut animated, &script, 1.0);
        assert_eq!(runner.finished, false);
        assert_eq!(runner.index, 0);

        runner.progress(Duration::from_millis(20), &mut animated, &script);
        assert_eq!(fixture.position.0, Point::new(1, 0));
        assert_eq!(runner.finished, false);

        // runner.progress(Duration::from_millis(40), &mut animated, &script);
        // assert_eq!(fixture.position.0, Point::new(1, 0));
        // assert_eq!(runner.finished, false);
    }
}
