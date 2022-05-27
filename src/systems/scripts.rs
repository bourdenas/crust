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
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
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

            if script.runner.state() == AnimationRunningState::Init {
                script.runner.start(&mut animated, &script.script);
            }

            if script.runner.state() == AnimationRunningState::Running {
                script
                    .runner
                    .progress(*data.time_since_last_frame, &mut animated, &script.script);
            }

            if script.runner.state() == AnimationRunningState::Finished {
                data.updater.remove::<ScriptState>(entity);
            }
        }
    }
}
