use super::{Animated, Performer};
use crate::{components::AnimationRunningState, crust::RotationAnimation};
use sdl2::rect::Point;

#[derive(Default)]
pub struct RotationPerformer {
    rotation: RotationAnimation,
    iteration: u32,
}

impl Performer for RotationPerformer {
    fn start(&mut self, animated: &mut Animated, _speed: f64) {
        if let Some(centre) = &self.rotation.centre {
            animated.rotation.centre = Some(Point::new(centre.x as i32, centre.y as i32));
        }
    }
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated) -> AnimationRunningState {
        animated.rotation.angle += self.rotation.angle;

        self.iteration += 1;
        match self.rotation.repeat > 0 && self.iteration == self.rotation.repeat {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}

impl RotationPerformer {
    pub fn new(rotation: RotationAnimation) -> Self {
        RotationPerformer {
            rotation,
            iteration: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{
            testing::util::Fixture, Performer, Progressor, ProgressorImpl, RotationPerformer,
        },
        components::AnimationRunningState,
        crust::RotationAnimation,
    };
    use sdl2::rect::Rect;
    use std::time::Duration;

    #[test]
    fn rotation_default_centre() {
        let mut fixture = Fixture::new();

        let animation = RotationAnimation {
            angle: 3.0,
            centre: None,
            delay: 100,
            repeat: 0,
        };

        // Test RotationPerformer.
        let mut performer = RotationPerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.position.0, Rect::new(0, 0, 32, 32));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.position.0, Rect::new(0, 0, 38, 64));

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = ProgressorImpl::new(
            RotationPerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.position.0, Rect::new(0, 0, 32, 32));
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(50), &mut animated),
            Duration::ZERO
        );
        assert_eq!(fixture.position.0, Rect::new(0, 0, 38, 64));
        assert_eq!(performer.finished(), true);
    }
}
