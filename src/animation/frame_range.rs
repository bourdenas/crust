use super::{Animated, Performer};
use crate::{
    components::{AnimationRunningState, Position, Sprite},
    crust::{Animation, HorizontalAlign, VerticalAlign},
    resources::SpriteSheet,
};
use sdl2::rect::Point;

#[derive(Default)]
pub struct FrameRangePerformer {
    step: i32,
    iteration: u32,
    finished: bool,
}

impl Performer for FrameRangePerformer {
    fn start(&mut self, animated: &mut Animated, animation: &Animation, speed: f64) {
        let frame_range = animation.frame_range.as_ref().unwrap();
        self.step = match frame_range.start_frame < frame_range.end_frame {
            true => 1,
            false => -1,
        };
        let start_frame = match speed < 0.0 {
            true => frame_range.end_frame - self.step,
            false => frame_range.start_frame,
        };

        set_frame(
            start_frame,
            VerticalAlign::from_i32(frame_range.vertical_align).unwrap(),
            HorizontalAlign::from_i32(frame_range.horizontal_align).unwrap(),
            animated.sprite,
            animated.position,
            animated.sprite_sheet,
        );

        self.finished =
            frame_range.repeat == 1 && (frame_range.start_frame - frame_range.end_frame).abs() == 1;
    }

    fn stop(&mut self, _animated: &mut Animated) {}
    fn pause(&mut self, _animated: &mut Animated) {}
    fn resume(&mut self, _animated: &mut Animated) {}

    fn execute(&mut self, animated: &mut Animated, animation: &Animation) -> AnimationRunningState {
        let frame_range = animation.frame_range.as_ref().unwrap();
        let mut next_frame = animated.sprite.frame_index as i32 + self.step;
        if next_frame == frame_range.end_frame {
            next_frame = frame_range.start_frame;
        }

        set_frame(
            next_frame,
            VerticalAlign::from_i32(frame_range.vertical_align).unwrap(),
            HorizontalAlign::from_i32(frame_range.horizontal_align).unwrap(),
            animated.sprite,
            animated.position,
            animated.sprite_sheet,
        );

        if animated.sprite.frame_index as i32 == frame_range.end_frame - self.step
            && frame_range.repeat > 0
        {
            self.iteration += 1;
            if self.iteration == frame_range.repeat {
                return AnimationRunningState::Finished;
            }
        }
        AnimationRunningState::Running
    }
}

impl FrameRangePerformer {
    pub fn new() -> Self {
        FrameRangePerformer::default()
    }
}

/// Handles sprite frame changes taking care of sprite film alignments.
fn set_frame(
    frame_index: i32,
    v_align: VerticalAlign,
    h_align: HorizontalAlign,
    sprite: &mut Sprite,
    position: &mut Position,
    sprite_sheet: &SpriteSheet,
) {
    let mut prev_aabb = sprite_sheet.bounding_boxes[sprite.frame_index as usize].clone();
    prev_aabb.reposition(position.0);
    let mut next_aabb = sprite_sheet.bounding_boxes[frame_index as usize].clone();
    next_aabb.reposition(position.0);

    sprite.frame_index = frame_index as usize;
    position.0 += Point::new(
        match h_align {
            HorizontalAlign::Right => {
                position.0.x() + (prev_aabb.width() - next_aabb.width()) as i32
            }
            HorizontalAlign::Hcentre => {
                position.0.x() + ((prev_aabb.width() - next_aabb.width()) / 2) as i32
            }
            _ => 0,
        },
        match v_align {
            VerticalAlign::Bottom => {
                position.0.y() + (prev_aabb.height() - next_aabb.height()) as i32
            }
            VerticalAlign::Vcentre => {
                position.0.y() + (prev_aabb.height() - next_aabb.height() / 2) as i32
            }
            _ => 0,
        },
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        animation::{performer::PerformerBase, testing::Fixture, FrameRangePerformer, Performer},
        components::AnimationRunningState,
        crust::{Animation, FrameRangeAnimation},
    };
    use std::time::Duration;

    #[test]
    fn single_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            frame_range: Some(FrameRangeAnimation {
                start_frame: 2,
                end_frame: 5,
                delay: 100,
                repeat: 1,
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(),
            Duration::from_millis(animation.frame_range.as_ref().unwrap().delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(50), &mut animated, &animation,),
            Duration::from_millis(50)
        );
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(180), &mut animated, &animation,),
            Duration::from_millis(150)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn repeated_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            frame_range: Some(FrameRangeAnimation {
                start_frame: 3,
                end_frame: 5,
                delay: 100,
                repeat: 2,
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Running
        );
        assert_eq!(fixture.sprite.frame_index, 3);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
            AnimationRunningState::Finished
        );
        assert_eq!(fixture.sprite.frame_index, 4);

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(),
            Duration::from_millis(animation.frame_range.as_ref().unwrap().delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(800), &mut animated, &animation,),
            Duration::from_millis(300)
        );
        assert_eq!(fixture.sprite.frame_index, 4);
        assert_eq!(performer.finished(), true);
    }

    #[test]
    fn indefinite_execution() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            frame_range: Some(FrameRangeAnimation {
                start_frame: 0,
                end_frame: 5,
                delay: 200,
                repeat: 0,
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 0);

        for i in 1..100 {
            let mut animated = fixture.animated();
            assert_eq!(
                performer.execute(&mut animated, &animation),
                AnimationRunningState::Running
            );
            assert_eq!(fixture.sprite.frame_index, i % 5);
        }

        // Test Performer using PerformerBase.
        let mut fixture = Fixture::new();
        let mut performer = PerformerBase::new(
            FrameRangePerformer::new(),
            Duration::from_millis(animation.frame_range.as_ref().unwrap().delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 0);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(200), &mut animated, &animation,),
            Duration::from_millis(200)
        );
        assert_eq!(fixture.sprite.frame_index, 1);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(1500), &mut animated, &animation,),
            Duration::from_millis(1500)
        );
        assert_eq!(fixture.sprite.frame_index, 3);
        assert_eq!(performer.finished(), false);
    }

    #[test]
    fn zero_delay() {
        let mut fixture = Fixture::new();

        let animation = Animation {
            frame_range: Some(FrameRangeAnimation {
                start_frame: 1,
                end_frame: 5,
                delay: 0,
                repeat: 0,
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut performer = FrameRangePerformer::new();
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 1);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.execute(&mut animated, &animation),
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
            FrameRangePerformer::new(),
            Duration::from_millis(animation.frame_range.as_ref().unwrap().delay as u64),
        );
        let mut animated = fixture.animated();
        performer.start(&mut animated, &animation, 1.0);
        assert_eq!(fixture.sprite.frame_index, 1);
        assert_eq!(performer.finished(), false);

        let mut animated = fixture.animated();
        assert_eq!(
            performer.progress(Duration::from_millis(200), &mut animated, &animation,),
            Duration::ZERO
        );
        assert_eq!(fixture.sprite.frame_index, 2);
        assert_eq!(performer.finished(), true);
    }
}
