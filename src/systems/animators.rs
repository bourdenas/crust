use crate::animation;
use crate::components::{AnimationState, FrameRange, Position, Sprite, Translation};
use crate::resources::SpriteSheetsManager;
use specs::prelude::*;
use std::time::Duration;

pub struct TranslationSystem;

impl<'a> System<'a> for TranslationSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Duration>,
        WriteStorage<'a, Translation>,
        WriteStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, time_since_last_frame, mut translation, mut position, updater): Self::SystemData,
    ) {
        for (entity, translation, position) in (&entities, &mut translation, &mut position).join() {
            let mut perfomer = animation::TranslationPerformer::new(translation, position);
            perfomer.run(&*time_since_last_frame);

            if translation.state == AnimationState::Finished {
                updater.remove::<Translation>(entity);
            }
        }
    }
}

pub struct FrameRangeSystem;

impl<'a> System<'a> for FrameRangeSystem {
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
            let mut perfomer =
                animation::FrameRangePerformer::new(sprite, position, frame_range, sprite_sheet);
            perfomer.run(&*time_since_last_frame);

            if frame_range.state == AnimationState::Finished {
                updater.remove::<FrameRange>(entity);
            }
        }
    }
}
