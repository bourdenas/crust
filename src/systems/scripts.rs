use crate::components::{AnimationState, FrameRange, Position, Script, Sprite, Translation};
use specs::prelude::*;
use specs::shred::Fetch;
use specs::storage::MaskedStorage;
use std::time::Duration;

#[derive(Default)]
pub struct ScriptSystem {
    pub finished: BitSet,
    pub translation_reader_id: Option<ReaderId<ComponentEvent>>,
    pub frame_range_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ScriptSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Duration>,
        WriteStorage<'a, Script>,
        ReadStorage<'a, Translation>,
        ReadStorage<'a, FrameRange>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, time_since_last_frame, mut script, translation, frame_range, updater): Self::SystemData,
    ) {
        self.collect_finished(translation, frame_range);

        for (entity, _) in (&entities, &self.finished).join() {
            updater.remove::<Translation>(entity);
            updater.remove::<FrameRange>(entity);
            // let mut perfomer = animation::TranslationPerformer::new(translation, position);
            // perfomer.run(&*time_since_last_frame);

            // if translation.state == AnimationState::Finished {
            //     updater.remove::<Translation>(entity);
            // }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.translation_reader_id =
            Some(WriteStorage::<Translation>::fetch(world).register_reader());
        self.frame_range_reader_id =
            Some(WriteStorage::<FrameRange>::fetch(world).register_reader());
    }
}

impl ScriptSystem {
    fn collect_finished(
        &mut self,
        translation: Storage<Translation, Fetch<MaskedStorage<Translation>>>,
        frame_range: Storage<FrameRange, Fetch<MaskedStorage<FrameRange>>>,
    ) {
        self.finished.clear();

        let events = translation
            .channel()
            .read(self.translation_reader_id.as_mut().unwrap());
        for event in events {
            match event {
                ComponentEvent::Removed(id) => {
                    println!("Removed translation with id={}", id);
                    self.finished.add(*id);
                }
                _ => (),
            }
        }

        let events = frame_range
            .channel()
            .read(self.frame_range_reader_id.as_mut().unwrap());
        for event in events {
            match event {
                ComponentEvent::Removed(id) => {
                    println!("Removed frame_range with id={}", id);
                    self.finished.add(*id);
                }
                _ => (),
            }
        }
    }
}
