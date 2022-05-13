use crate::components::{
    AnimationRunningState, FrameRangeState, ScriptState, TimerState, TranslationState,
};
use specs::prelude::*;

#[derive(SystemData)]
pub struct ScriptSystemData<'a> {
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,
    // _time_since_last_time: ReadExpect<'a, Duration>,
    scripts: WriteStorage<'a, ScriptState>,
    translations: ReadStorage<'a, TranslationState>,
    frame_ranges: ReadStorage<'a, FrameRangeState>,
    timers: ReadStorage<'a, TimerState>,
}

#[derive(Default)]
pub struct ScriptSystem {
    pub finished: BitSet,
    pub translation_reader_id: Option<ReaderId<ComponentEvent>>,
    pub frame_range_reader_id: Option<ReaderId<ComponentEvent>>,
    pub timer_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ScriptSystem {
    type SystemData = ScriptSystemData<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.translation_reader_id =
            Some(WriteStorage::<TranslationState>::fetch(world).register_reader());
        self.frame_range_reader_id =
            Some(WriteStorage::<FrameRangeState>::fetch(world).register_reader());
        self.timer_reader_id = Some(WriteStorage::<TimerState>::fetch(world).register_reader());
    }

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;
        for (entity, script) in (&data.entities, &mut data.scripts).join() {
            if script.state == AnimationRunningState::Init {
                self.play_next(entity, script, &data.updater);
            }
        }

        self.collect_finished(&data.translations, &data.frame_ranges, &data.timers);
        self.progress(&mut data);
    }
}

impl ScriptSystem {
    fn progress(&mut self, data: &mut ScriptSystemData) {
        for (entity, script, translation, frame_range, timer, _) in (
            &data.entities,
            &mut data.scripts,
            (&data.translations).maybe(),
            (&data.frame_ranges).maybe(),
            (&data.timers).maybe(),
            &self.finished,
        )
            .join()
        {
            let print = |(x, y, z): &(
                Option<&TranslationState>,
                Option<&FrameRangeState>,
                Option<&TimerState>,
            )| {
                format!(
                    "({}, {},{})",
                    match x {
                        Some(_) => "Trans",
                        None => "None",
                    },
                    match y {
                        Some(_) => "Frame",
                        None => "None",
                    },
                    match z {
                        Some(z) => format!("{:?}", z),
                        None => "None".to_owned(),
                    },
                )
            };

            let animation = &script.script.animation[script.index - 1];
            let animations = (translation, frame_range, timer);
            println!("animation: {:?}", print(&animations));
            let finished = match animations {
                (None, None, None) => true,
                _ if !animation.wait_all => true,
                _ => false,
            };

            if !finished {
                continue;
            }

            if script.index < script.script.animation.len() {
                self.play_next(entity, script, &data.updater);
            } else if script.script.repeat == 0
                || script.iteration + 1 < script.script.repeat as u32
            {
                script.iteration += 1;
                script.index = 0;
                self.play_next(entity, script, &data.updater);
            } else {
                self.stop(entity, script, &data.updater);
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

        if let Some(animation) = &animation.timer {
            updater.insert(entity, TimerState::new(animation.clone()));
        } else {
            updater.remove::<TimerState>(entity);
        }

        script.state = AnimationRunningState::Running;
        script.index += 1;
    }

    fn stop(&self, entity: Entity, script: &mut ScriptState, updater: &Read<LazyUpdate>) {
        updater.remove::<TranslationState>(entity);
        updater.remove::<FrameRangeState>(entity);
        updater.remove::<TimerState>(entity);
        script.state = AnimationRunningState::Finished;
    }

    fn collect_finished(
        &mut self,
        translation: &ReadStorage<TranslationState>,
        frame_range: &ReadStorage<FrameRangeState>,
        timer: &ReadStorage<TimerState>,
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

        let events = timer.channel().read(self.timer_reader_id.as_mut().unwrap());
        for event in events {
            match event {
                ComponentEvent::Removed(id) => {
                    println!("timer removed");
                    self.finished.add(*id)
                }
                _ => false,
            };
        }
    }
}
