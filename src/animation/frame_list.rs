use super::{Animated, Performer};
use crate::{
    components::AnimationRunningState,
    crust::{FrameListAnimation, HorizontalAlign, VerticalAlign},
};

#[derive(Default)]
pub struct FrameListPerformer {
    frame_list: FrameListAnimation,
    index: usize,
    iteration: u32,
    finished: bool,
}

impl Performer for FrameListPerformer {
    fn start(&mut self, animated: &mut Animated, speed: f64) {
        self.index = match speed < 0.0 {
            false => 0,
            true => self.frame_list.frame.len() - 1,
        };

        animated.change_frame(
            self.frame_list.frame[self.index] as usize,
            VerticalAlign::from_i32(self.frame_list.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_list.horizontal_align).unwrap(),
        );

        self.finished = self.frame_list.repeat == 1 && self.frame_list.frame.len() == 1;
    }

    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated) -> AnimationRunningState {
        self.index += 1;
        if self.index == self.frame_list.frame.len() {
            self.index = 0;
        }

        animated.change_frame(
            self.frame_list.frame[self.index] as usize,
            VerticalAlign::from_i32(self.frame_list.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_list.horizontal_align).unwrap(),
        );

        if self.index == (self.frame_list.frame.len() - 1) && self.frame_list.repeat > 0 {
            self.iteration += 1;
            if self.iteration == self.frame_list.repeat {
                return AnimationRunningState::Finished;
            }
        }
        AnimationRunningState::Running
    }
}

impl FrameListPerformer {
    pub fn new(frame_list: FrameListAnimation) -> Self {
        FrameListPerformer {
            frame_list,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{
            testing::util::Fixture, FrameListPerformer, Performer, Progressor, ProgressorImpl,
        },
        components::AnimationRunningState,
        crust::FrameListAnimation,
    };
    use std::time::Duration;

    #[test]
    fn single_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameListAnimation {
            frame: vec![2, 4, 2, 5],
            delay: 100,
            repeat: 1,
            ..Default::default()
        };

        // Test FrameListPerformer.
        let mut performer = FrameListPerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);

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
        assert_eq!(fixture.sprite.frame_index, 2);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 5);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = ProgressorImpl::new(
            FrameListPerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(150), &mut animated,),
            Duration::from_millis(150)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(180), &mut animated,),
            Duration::from_millis(150)
        );
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn repeated_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameListAnimation {
            frame: vec![3, 1, 4],
            delay: 100,
            repeat: 2,
            ..Default::default()
        };

        let mut performer = FrameListPerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 1);

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
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 1);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = ProgressorImpl::new(
            FrameListPerformer::new(animation.clone()),
            Duration::from_millis(animation.delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(800), &mut animated,),
            Duration::from_millis(500)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn indefinite_execution() {
        let mut fixture = Fixture::new();

        let animation = FrameListAnimation {
            frame: vec![0, 1, 2, 3, 4],
            delay: 200,
            repeat: 0,
            ..Default::default()
        };

        let mut performer = FrameListPerformer::new(animation.clone());
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
        let mut performer = ProgressorImpl::new(
            FrameListPerformer::new(animation.clone()),
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

        let animation = FrameListAnimation {
            frame: vec![1, 5, 3, 2],
            delay: 0,
            repeat: 0,
            ..Default::default()
        };

        let mut performer = FrameListPerformer::new(animation.clone());
        let mut animated = fixture.animated();
        performer.start(&mut animated, 1.0);
        assert_eq!(fixture.sprite.frame_index, 1);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 5);

        // Test Performer using PerformerBase.
        //
        // This is a corner case that makes little sense and doesn't have a
        // clear correct behaviour. Implemented behaviour is that animation will
        // apply only one frame change and finish after that.
        let mut fixture = Fixture::new();
        let mut performer = ProgressorImpl::new(
            FrameListPerformer::new(animation.clone()),
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
        assert_eq!(fixture.sprite.frame_index, 5);
        assert_eq!(performer.finished(), true);
    }
}
