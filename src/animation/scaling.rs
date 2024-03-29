use super::{Animated, Performer};
use crate::{components::AnimationRunningState, crust::VectorAnimation};

#[derive(Default)]
pub struct ScalingPerformer {
    scaling: VectorAnimation,
    iteration: u32,
}

impl Performer for ScalingPerformer {
    fn start(&mut self, _animated: &mut Animated, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated) -> AnimationRunningState {
        if let Some(vec) = &self.scaling.vec {
            animated.scaling.0 = (vec.x, vec.y);

            animated.position.0.resize(
                (animated.position.0.width() as f64 * vec.x) as u32,
                (animated.position.0.height() as f64 * vec.y) as u32,
            );
        }

        self.iteration += 1;
        match self.scaling.repeat > 0 && self.iteration == self.scaling.repeat {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}

impl ScalingPerformer {
    pub fn new(scaling: VectorAnimation) -> Self {
        ScalingPerformer {
            scaling,
            iteration: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{
            testing::util::Fixture, Performer, Progressor, ProgressorImpl, ScalingPerformer,
        },
        components::AnimationRunningState,
        crust::{Vector, VectorAnimation},
    };
    use sdl2::rect::Rect;
    use std::time::Duration;

    #[test]
    fn zero_delay() {
        let mut fixture = Fixture::new();

        let animation = VectorAnimation {
            vec: Some(Vector {
                x: 1.2,
                y: 2.0,
                ..Default::default()
            }),
            delay: 0,
            repeat: 1,
        };

        // Test ScalingPerformer.
        let mut performer = ScalingPerformer::new(animation.clone());
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
            ScalingPerformer::new(animation.clone()),
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
