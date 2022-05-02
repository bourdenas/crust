use crate::components::{AnimationState, FrameRange, Position, Sprite};
use crate::resources::{SpriteSheet, SpriteSheetsManager};
use crate::trust::{HorizontalAlign, VerticalAlign};
use sdl2::rect::Point;
use specs::prelude::*;
use std::time::Duration;

pub struct FrameRangePerformer;

impl<'a> System<'a> for FrameRangePerformer {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Duration>,
        ReadExpect<'a, SpriteSheetsManager>,
        WriteStorage<'a, FrameRange>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            time_since_last_frame,
            sheets_manager,
            mut frame_range,
            mut sprite,
            mut position,
            updater,
        ): Self::SystemData,
    ) {
        for (entity, frame_range, sprite, position) in
            (&entities, &mut frame_range, &mut sprite, &mut position).join()
        {
            // println!("time since last frame {:?}", &*time_since_last_frame);
            let sprite_sheet = &sheets_manager.load(&sprite.resource).unwrap();

            if frame_range.state == AnimationState::Init {
                self.start(sprite, position, frame_range, sprite_sheet)
            }
            if frame_range.state == AnimationState::Running {
                self.progress(
                    &*time_since_last_frame,
                    sprite,
                    position,
                    sprite_sheet,
                    frame_range,
                );
            }
            if frame_range.state == AnimationState::Finished {
                updater.remove::<FrameRange>(entity);
            }
        }
    }
}

impl FrameRangePerformer {
    fn start(
        &self,
        sprite: &mut Sprite,
        position: &mut Position,
        frame_range: &mut FrameRange,
        sprite_sheet: &SpriteSheet,
    ) {
        let start_frame = match frame_range.speed < 0.0 {
            true => frame_range.animation.end_frame - frame_range.step,
            false => frame_range.animation.start_frame,
        };

        set_frame(
            start_frame,
            VerticalAlign::from_i32(frame_range.animation.vertical_align).unwrap(),
            HorizontalAlign::from_i32(frame_range.animation.horizontal_align).unwrap(),
            sprite,
            position,
            sprite_sheet,
        );

        frame_range.state = match frame_range.animation.repeat == 1
            && (frame_range.animation.start_frame - frame_range.animation.end_frame).abs() == 1
        {
            true => AnimationState::Finished,
            false => AnimationState::Running,
        }
    }

    fn progress(
        &mut self,
        time_since_last_frame: &Duration,
        sprite: &mut Sprite,
        position: &mut Position,
        sprite_sheet: &SpriteSheet,
        frame_range: &mut FrameRange,
    ) -> Duration {
        if frame_range.animation.delay == 0 {
            self.execute(sprite, position, frame_range, sprite_sheet);
            frame_range.state = AnimationState::Finished;
            return Duration::ZERO;
        }

        frame_range.wait_time += *time_since_last_frame;
        let animation_delay = Duration::from_millis(frame_range.animation.delay as u64);
        while animation_delay < frame_range.wait_time {
            frame_range.wait_time -= animation_delay;
            if let AnimationState::Finished =
                self.execute(sprite, position, frame_range, sprite_sheet)
            {
                frame_range.state = AnimationState::Finished;
                return *time_since_last_frame - frame_range.wait_time;
            }
        }

        *time_since_last_frame
    }

    fn execute(
        &self,
        sprite: &mut Sprite,
        position: &mut Position,
        frame_range: &mut FrameRange,
        sprite_sheet: &SpriteSheet,
    ) -> AnimationState {
        let next_frame = sprite.frame_index as i32 + frame_range.step;
        let next_frame = frame_range.animation.start_frame
            + next_frame
                % (frame_range.animation.start_frame - frame_range.animation.end_frame).abs();

        set_frame(
            next_frame,
            VerticalAlign::from_i32(frame_range.animation.vertical_align).unwrap(),
            HorizontalAlign::from_i32(frame_range.animation.horizontal_align).unwrap(),
            sprite,
            position,
            sprite_sheet,
        );

        if sprite.frame_index as i32 == frame_range.animation.end_frame - frame_range.step
            && frame_range.animation.repeat > 0
        {
            frame_range.run_number += 1;
            if frame_range.run_number == frame_range.animation.repeat {
                return AnimationState::Finished;
            }
        }
        AnimationState::Running
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
