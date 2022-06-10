use crate::{
    action::ActionQueue,
    animation::Animated,
    components::{AnimationRunningState, Id, Position, ScriptState, Sprite, Velocity},
    crust::{event, AnimationEvent, Vector},
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

    ids: ReadStorage<'a, Id>,
    scripts: WriteStorage<'a, ScriptState>,
    positions: ReadStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
    sprites: WriteStorage<'a, Sprite>,
}

pub struct ScriptSystem {
    queue: ActionQueue,
}

impl<'a> System<'a> for ScriptSystem {
    type SystemData = ScriptSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for (entity, id, script, position, velocity, sprite) in (
            &data.entities,
            &data.ids,
            &mut data.scripts,
            &data.positions,
            &mut data.velocities,
            &mut data.sprites,
        )
            .join()
        {
            let sprite_sheet = &data.sheets_manager.load(&sprite.resource).unwrap();
            let mut animated = Animated::new(
                entity,
                id,
                position,
                velocity,
                sprite,
                sprite_sheet,
                Some(&self.queue),
            );

            if script.runner.state() == AnimationRunningState::Init {
                script.runner.start(&mut animated);
            }

            if script.runner.state() == AnimationRunningState::Running {
                script
                    .runner
                    .progress(*data.time_since_last_frame, &mut animated);
            }

            if script.runner.state() == AnimationRunningState::Finished {
                self.emit_done(id, script, position, sprite);
                data.updater.remove::<ScriptState>(entity);
            }
        }
    }
}

impl ScriptSystem {
    pub fn new(queue: ActionQueue) -> Self {
        ScriptSystem { queue }
    }

    fn emit_done(&self, id: &Id, script: &ScriptState, position: &Position, sprite: &Sprite) {
        self.queue.emit(
            format!("{}_script_done", id.0),
            event::Event::AnimationScriptDone(AnimationEvent {
                animation_id: script.runner.script.id.clone(),
                position: Some(Vector {
                    x: position.0.x() as f64,
                    y: position.0.y() as f64,
                    z: 0.0,
                }),
                frame_index: sprite.frame_index as u32,
            }),
        );
    }
}
