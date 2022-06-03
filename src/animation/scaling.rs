use super::{Animated, Performer};
use crate::{
    components::{AnimationRunningState, ScalingVec},
    crust::VectorAnimation,
};

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
            animated.sprite.scaling *= ScalingVec::new(vec.x, vec.y);
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
        animation::{performer::PerformerBase, testing::Fixture, Performer, ScalingPerformer},
        components::{AnimationRunningState, ScalingVec},
        crust::{Vector, VectorAnimation},
    };
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
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.0, 1.0));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.2, 2.0));

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            ScalingPerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.0, 1.0));
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(50), &mut animated),
            Duration::ZERO
        );
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.2, 2.0));
        assert_eq!(performer.finished(), true);
    }
}
