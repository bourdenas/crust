use super::{Animated, Performer};
use crate::{components::AnimationRunningState, trust::Animation};
use sdl2::rect::Point;

#[derive(Default)]
pub struct TranslationPerformer {
    iteration: u32,
}

impl Performer for TranslationPerformer {
    fn start(&mut self, _animated: &mut Animated, _animation: &Animation, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState {
        let translation = animation.translation.as_ref().unwrap();
        if let Some(vec) = &translation.vec {
            animated.position.0 += Point::new(vec.x as i32, vec.y as i32);
        }

        self.iteration += 1;
        match translation.repeat > 0 && self.iteration == translation.repeat {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}

impl TranslationPerformer {
    pub fn new() -> Self {
        TranslationPerformer::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{testing::Fixture, Performer, TranslationPerformer},
        components::AnimationRunningState,
        trust::{Animation, Vector, VectorAnimation},
    };
    use sdl2::rect::Point;

    #[test]
    fn single_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            translation: Some(VectorAnimation {
                vec: Some(Vector {
                    x: 1.0,
                    ..Default::default()
                }),
                delay: 20,
                repeat: 1,
            }),
            ..Default::default()
        };

        let mut performer = TranslationPerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.position.0, Point::new(0, 0));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.position.0, Point::new(1, 0));
    }

    #[test]
    fn repeated_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            translation: Some(VectorAnimation {
                vec: Some(Vector {
                    x: 1.0,
                    y: 1.0,
                    ..Default::default()
                }),
                delay: 20,
                repeat: 3,
            }),
            ..Default::default()
        };

        let mut performer = TranslationPerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.position.0, Point::new(0, 0));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.position.0, Point::new(1, 1));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.position.0, Point::new(2, 2));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.position.0, Point::new(3, 3));
    }

    #[test]
    fn indefinite_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            translation: Some(VectorAnimation {
                vec: Some(Vector {
                    y: 2.0,
                    ..Default::default()
                }),
                delay: 20,
                repeat: 0,
            }),
            ..Default::default()
        };

        let mut performer = TranslationPerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.position.0, Point::new(0, 0));

        for i in 1..100 {
            let mut animated = fixture.animated();
            assert_eq!(
                performer.execute(&mut animated, &animation),
                AnimationRunningState::Running
            );
            assert_eq!(fixture.position.0, Point::new(0, i * 2));
        }
    }
}
