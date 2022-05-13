use crate::components::{AnimationRunningState, FrameRangeState, ScriptState, TranslationState};
use crate::trust::Animation;
use specs::prelude::*;
use specs::world::EntitiesRes;
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
                self.start_animation(entity, &script.script.animation[script.index], &updater);
                script.state = AnimationRunningState::Running;
                script.index += 1;
            }
        }

        self.progress(&entities, &mut script, &updater);
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
    fn progress(
        &mut self,
        entities: &Read<EntitiesRes>,
        script: &mut WriteStorage<ScriptState>,
        updater: &Read<LazyUpdate>,
    ) {
        for (entity, script, _) in (entities, script, &self.finished).join() {
            if script.index < script.script.animation.len() {
                self.start_animation(entity, &script.script.animation[script.index], updater);
                script.index += 1;
            } else {
                self.stop_animation(entity, updater);
            }
        }
    }

    fn start_animation(&self, entity: Entity, animation: &Animation, updater: &Read<LazyUpdate>) {
        if let Some(animation) = &animation.translation {
            updater.insert(entity, TranslationState::new(animation.clone()));
        } else {
            updater.remove::<TranslationState>(entity);
        }
        if let Some(animation) = &animation.frame_range {
            updater.insert(entity, FrameRangeState::new(animation.clone()));
        } else {
            updater.remove::<FrameRangeState>(entity);
        }
    }

    fn stop_animation(&self, entity: Entity, updater: &Read<LazyUpdate>) {
        updater.remove::<TranslationState>(entity);
        updater.remove::<FrameRangeState>(entity);
    }

    fn collect_finished(
        &mut self,
        translation: ReadStorage<TranslationState>,
        frame_range: ReadStorage<FrameRangeState>,
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
