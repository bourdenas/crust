use super::{Animated, Performer};
use crate::{
    components::{AnimationRunningState, Position, Sprite},
    resources::SpriteSheet,
    trust::{Animation, HorizontalAlign, VerticalAlign},
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
        let next_frame = animated.sprite.frame_index as i32 + self.step;
        let next_frame = frame_range.start_frame
            + next_frame % (frame_range.start_frame - frame_range.end_frame).abs();

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
