use super::{animated::set_frame, Animated, Performer};
use crate::{
    components::AnimationRunningState,
    crust::{FrameRangeAnimation, HorizontalAlign, VerticalAlign},
};

#[derive(Default)]
pub struct FrameRangePerformer {
    frame_range: FrameRangeAnimation,
    step: i32,
    iteration: u32,
    finished: bool,
}

impl Performer for FrameRangePerformer {
    fn start(&mut self, animated: &mut Animated, speed: f64) {
        self.step = match self.frame_range.start_frame < self.frame_range.end_frame {
            true => 1,
            false => -1,
        };
        let start_frame = match speed < 0.0 {
            true => self.frame_range.end_frame - self.step,
            false => self.frame_range.start_frame,
        };

        set_frame(
            start_frame as usize,
            VerticalAlign::from_i32(self.frame_range.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_range.horizontal_align).unwrap(),
            animated.sprite,
            animated.position,
            animated.velocity,
            animated.sprite_sheet,
        );

        self.finished = self.frame_range.repeat == 1
            && (self.frame_range.start_frame - self.frame_range.end_frame).abs() == 1;
    }

    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated) -> AnimationRunningState {
        let mut next_frame = animated.sprite.frame_index as i32 + self.step;
        if next_frame == self.frame_range.end_frame {
            next_frame = self.frame_range.start_frame;
        }

        set_frame(
            next_frame as usize,
            VerticalAlign::from_i32(self.frame_range.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_range.horizontal_align).unwrap(),
            animated.sprite,
            animated.position,
            animated.velocity,
            animated.sprite_sheet,
        );

        if animated.sprite.frame_index as i32 == self.frame_range.end_frame - self.step
            && self.frame_range.repeat > 0
        {
            self.iteration += 1;
            if self.iteration == self.frame_range.repeat {
                return AnimationRunningState::Finished;
            }
        }
        AnimationRunningState::Running
    }
}

impl FrameRangePerformer {
    pub fn new(frame_range: FrameRangeAnimation) -> Self {
        FrameRangePerformer {
            frame_range,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{
            performer::PerformerBase, testing::util::Fixture, FrameRangePerformer, Performer,
        },
        components::AnimationRunningState,
        crust::FrameRangeAnimation,
    };
    use std::time::Duration;

    #[test]
    fn single_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameRangeAnimation {
            start_frame: 2,
            end_frame: 5,
            delay: 100,
            repeat: 1,
            ..Default::default()
        };

        // Test FrameRangePerformer.
        let mut performer = FrameRangePerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(50), &mut animated,),
            Duration::from_millis(50)
        );
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(180), &mut animated,),
            Duration::from_millis(150)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn repeated_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameRangeAnimation {
            start_frame: 3,
            end_frame: 5,
            delay: 100,
            repeat: 2,
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(800), &mut animated,),
            Duration::from_millis(300)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn indefinite_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameRangeAnimation {
            start_frame: 0,
            end_frame: 5,
            delay: 200,
            repeat: 0,
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 0);

        for i in 1..100 {
            let mut animated = fixture.animated();
            assert_eq!(
                performer.execute(&mut animated),
                AnimationRunningState::Running
            );
            assert_eq!(fixture.sprite.frame_index, i % 5);
        }

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 0);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(200), &mut animated,),
            Duration::from_millis(200)
        );
        assert_eq!(fixture.sprite.frame_index, 1);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(1500), &mut animated,),
            Duration::from_millis(1500)
        );
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(performer.finished(), false);
    }

    #[test]
    fn zero_delay() {
        let mut fixture = Fixture::new();

        let animation = FrameRangeAnimation {
            start_frame: 1,
            end_frame: 5,
            delay: 0,
            repeat: 0,
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 1);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 2);

        // Test Performer using PerformerBase.
        //
        // This is a corner case that makes little sense and doesn't have a
        // clear correct behaviour. Implemented behaviour is that animation will
        // apply only one frame change and finish after that.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 1);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(200), &mut animated,),
            Duration::ZERO
        );
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), true);
    }
}
