use super::{Animated, Performer};
use crate::{
    components::{AnimationRunningState, ScalingVec},
    crust::Animation,
};

#[derive(Default)]
pub struct ScalingPerformer {
    iteration: u32,
}

impl Performer for ScalingPerformer {
    fn start(&mut self, _animated: &mut Animated, _animation: &Animation, _speed: f64) {}
    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState {
        let scaling = animation.scaling.as_ref().unwrap();
        if let Some(vec) = &scaling.vec {
            animated.sprite.scaling *= ScalingVec::new(vec.x, vec.y);
        }

        self.iteration += 1;
        match scaling.repeat > 0 && self.iteration == scaling.repeat {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }
}

impl ScalingPerformer {
    pub fn new() -> Self {
        ScalingPerformer::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{performer::PerformerBase, testing::Fixture, Performer, ScalingPerformer},
        components::{AnimationRunningState, ScalingVec},
        crust::{Animation, Vector, VectorAnimation},
    };
    use std::time::Duration;

    #[test]
    fn zero_delay() {
        let mut fixture = Fixture::new();

        let animation = Animation {
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
        };

        let mut performer = ScalingPerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.0, 1.0));

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.2, 2.0));

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            ScalingPerformer::new(),
            Duration::from_millis(animation.scaling.as_ref().unwrap().delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.0, 1.0));
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(50), &mut animated, &animation,),
            Duration::ZERO
        );
        assert_eq!(fixture.sprite.scaling, ScalingVec::new(1.2, 2.0));
        assert_eq!(performer.finished(), true);
    }
}
