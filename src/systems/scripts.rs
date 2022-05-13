use crate::components::{AnimationRunningState, FrameRangeState, ScriptState, TranslationState};
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.translation_reader_id =
            Some(WriteStorage::<TranslationState>::fetch(world).register_reader());
        self.frame_range_reader_id =
            Some(WriteStorage::<FrameRangeState>::fetch(world).register_reader());
    }

    fn run(
        &mut self,
        (entities, _time_since_last_frame, mut script, translation, frame_range, updater): Self::SystemData,
    ) {
        for (entity, script) in (&entities, &mut script).join() {
            if script.state == AnimationRunningState::Init {
                self.play_next(entity, script, &updater);
            }
        }

        self.collect_finished(&translation, &frame_range);
        self.progress(&entities, &mut script, &translation, &frame_range, &updater);
    }
}

impl ScriptSystem {
    fn progress(
        &mut self,
        entities: &Read<EntitiesRes>,
        script: &mut WriteStorage<ScriptState>,
        translation: &ReadStorage<TranslationState>,
        frame_range: &ReadStorage<FrameRangeState>,
        updater: &Read<LazyUpdate>,
    ) {
        for (entity, script, translation, frame_range, _) in (
            entities,
            script,
            (&translation).maybe(),
            (&frame_range).maybe(),
            &self.finished,
        )
            .join()
        {
            let animation = &script.script.animation[script.index - 1];
            let animations = (translation, frame_range);
            let finished = match animations {
                (None, None) => true,
                _ if !animation.wait_all => true,
                _ => false,
            };

            if !finished {
                continue;
            }

            if script.index < script.script.animation.len() {
                self.play_next(entity, script, updater);
            } else if script.script.repeat == 0
                || script.iteration + 1 < script.script.repeat as u32
            {
                script.iteration += 1;
                script.index = 0;
                self.play_next(entity, script, updater);
            } else {
                self.stop(entity, script, updater);
            }
        }
    }

    fn play_next(&self, entity: Entity, script: &mut ScriptState, updater: &Read<LazyUpdate>) {
        let animation = &script.script.animation[script.index];

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

        script.state = AnimationRunningState::Running;
        script.index += 1;
    }

    fn stop(&self, entity: Entity, script: &mut ScriptState, updater: &Read<LazyUpdate>) {
        updater.remove::<TranslationState>(entity);
        updater.remove::<FrameRangeState>(entity);
        script.state = AnimationRunningState::Finished;
    }

    fn collect_finished(
        &mut self,
        translation: &ReadStorage<TranslationState>,
        frame_range: &ReadStorage<FrameRangeState>,
    ) {
        self.finished.clear();

        let events = translation
            .channel()
            .read(self.translation_reader_id.as_mut().unwrap());
        for event in events {
            match event {
                ComponentEvent::Removed(id) => self.finished.add(*id),
                _ => false,
            };
        }

        let events = frame_range
            .channel()
            .read(self.frame_range_reader_id.as_mut().unwrap());
        for event in events {
            match event {
                ComponentEvent::Removed(id) => self.finished.add(*id),
                _ => false,
            };
        }
    }
}
