use crate::components::{AnimationRunningState, FrameRangeState, Position, Sprite};
use crate::resources::SpriteSheet;
use crate::trust::{HorizontalAlign, VerticalAlign};
use core::time::Duration;
use sdl2::rect::Point;

pub struct FrameRangePerformer<'a> {
    sprite: &'a mut Sprite,
    position: &'a mut Position,
    frame_range: &'a mut FrameRangeState,
    sprite_sheet: &'a SpriteSheet,
}

impl<'a> FrameRangePerformer<'a> {
    pub fn new(
        sprite: &'a mut Sprite,
        position: &'a mut Position,
        frame_range: &'a mut FrameRangeState,
        sprite_sheet: &'a SpriteSheet,
    ) -> Self {
        FrameRangePerformer {
            sprite,
            position,
            frame_range,
            sprite_sheet,
        }
    }

    pub fn run(&mut self, time_since_last_frame: &Duration) {
        if self.frame_range.state == AnimationRunningState::Init {
            self.start();
        }
        if self.frame_range.state == AnimationRunningState::Running {
            self.progress(&*time_since_last_frame);
        }
    }

    fn start(&mut self) {
        let start_frame = match self.frame_range.speed < 0.0 {
            true => self.frame_range.animation.end_frame - self.frame_range.step,
            false => self.frame_range.animation.start_frame,
        };

        set_frame(
            start_frame,
            VerticalAlign::from_i32(self.frame_range.animation.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_range.animation.horizontal_align).unwrap(),
            self.sprite,
            self.position,
            self.sprite_sheet,
        );

        self.frame_range.state = match self.frame_range.animation.repeat == 1
            && (self.frame_range.animation.start_frame - self.frame_range.animation.end_frame).abs()
                == 1
        {
            true => AnimationRunningState::Finished,
            false => AnimationRunningState::Running,
        }
    }

    fn progress(&mut self, time_since_last_frame: &Duration) -> Duration {
        if self.frame_range.animation.delay == 0 {
            self.execute();
            self.frame_range.state = AnimationRunningState::Finished;
            return Duration::ZERO;
        }

        self.frame_range.wait_time += *time_since_last_frame;
        let animation_delay = Duration::from_millis(self.frame_range.animation.delay as u64);
        while animation_delay <= self.frame_range.wait_time {
            self.frame_range.wait_time -= animation_delay;
            if let AnimationRunningState::Finished = self.execute() {
                self.frame_range.state = AnimationRunningState::Finished;
                return *time_since_last_frame - self.frame_range.wait_time;
            }
        }

        *time_since_last_frame
    }

    fn execute(&mut self) -> AnimationRunningState {
        let next_frame = self.sprite.frame_index as i32 + self.frame_range.step;
        let next_frame = self.frame_range.animation.start_frame
            + next_frame
                % (self.frame_range.animation.start_frame - self.frame_range.animation.end_frame)
                    .abs();

        set_frame(
            next_frame,
            VerticalAlign::from_i32(self.frame_range.animation.vertical_align).unwrap(),
            HorizontalAlign::from_i32(self.frame_range.animation.horizontal_align).unwrap(),
            self.sprite,
            self.position,
            self.sprite_sheet,
        );

        if self.sprite.frame_index as i32
            == self.frame_range.animation.end_frame - self.frame_range.step
            && self.frame_range.animation.repeat > 0
        {
            self.frame_range.run_number += 1;
            if self.frame_range.run_number == self.frame_range.animation.repeat {
                return AnimationRunningState::Finished;
            }
        }
        AnimationRunningState::Running
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
