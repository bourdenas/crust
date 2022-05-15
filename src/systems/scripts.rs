use crate::{
    animation::Animated,
    components::{AnimationRunningState, Position, ScriptState, Sprite},
    resources::SpriteSheetsManager,
};
use specs::prelude::*;
use std::time::Duration;

#[derive(SystemData)]
pub struct ScriptSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
    sheets_manager: ReadExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,

    scripts: WriteStorage<'a, ScriptState>,
    positions: WriteStorage<'a, Position>,
    sprites: WriteStorage<'a, Sprite>,
}

#[derive(Default)]
pub struct ScriptSystem;

impl<'a> System<'a> for ScriptSystem {
    type SystemData = ScriptSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for (entity, script, mut position, mut sprite) in (
            &data.entities,
            &mut data.scripts,
            &mut data.positions,
            &mut data.sprites,
        )
            .join()
        {
            let sprite_sheet = &data.sheets_manager.load(&sprite.resource).unwrap();
            let mut animated = Animated::new(entity, &mut position, &mut sprite, sprite_sheet);

            if script.state == AnimationRunningState::Init {
                script
                    .runner
                    .start(&mut animated, &script.script, script.speed);
                script.state = AnimationRunningState::Running;
            }

            if script.state == AnimationRunningState::Running {
                self.progress(*data.time_since_last_frame, &mut animated, script);
            }

            if script.state == AnimationRunningState::Finished {
                data.updater.remove::<ScriptState>(entity);
            }
        }
    }
}

impl ScriptSystem {
    fn progress(
        &mut self,
        time_since_last_frame: Duration,
        animated: &mut Animated,
        script: &mut ScriptState,
    ) {
        let time_consumed = script
            .runner
            .progress(time_since_last_frame, animated, &script.script);
        if script.runner.finished() {
            script.iteration += 1;
            if script.script.repeat == 0 || script.iteration < script.script.repeat {
                // TODO: emit rewind event
                script.runner.start(animated, &script.script, script.speed);
                self.progress(time_since_last_frame - time_consumed, animated, script);
            } else {
                script.state = AnimationRunningState::Finished;
            }
        }
    }
}
