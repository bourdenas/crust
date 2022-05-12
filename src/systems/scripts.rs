use crate::components::{AnimationRunningState, FrameRangeState, ScriptState, TranslationState};
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
        WriteStorage<'a, ScriptState>,
        ReadStorage<'a, TranslationState>,
        ReadStorage<'a, FrameRangeState>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, _time_since_last_frame, mut script, translation, frame_range, updater): Self::SystemData,
    ) {
        self.collect_finished(translation, frame_range);

        for (entity, script) in (&entities, &mut script).join() {
            if script.state == AnimationRunningState::Init {
                let animation = &script.script.animation[script.index];
                if let Some(animation) = &animation.translation {
                    updater.insert(entity, TranslationState::new(animation.clone()));
                }
                if let Some(animation) = &animation.frame_range {
                    updater.insert(entity, FrameRangeState::new(animation.clone()));
                }
                script.state = AnimationRunningState::Running;
            }
        }

        for (entity, _) in (&entities, &self.finished).join() {
            updater.remove::<TranslationState>(entity);
            updater.remove::<FrameRangeState>(entity);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.translation_reader_id =
            Some(WriteStorage::<TranslationState>::fetch(world).register_reader());
        self.frame_range_reader_id =
            Some(WriteStorage::<FrameRangeState>::fetch(world).register_reader());
    }
}

impl ScriptSystem {
    fn collect_finished(
        &mut self,
        translation: Storage<TranslationState, Fetch<MaskedStorage<TranslationState>>>,
        frame_range: Storage<FrameRangeState, Fetch<MaskedStorage<FrameRangeState>>>,
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
