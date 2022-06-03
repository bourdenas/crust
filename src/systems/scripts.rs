use crate::{
    animation::Animated,
    components::{AnimationRunningState, Id, Position, ScriptState, Sprite},
    crust::{action, event, Action, AnimationEvent, EmitAction, Event, Vector},
    resources::SpriteSheetsManager,
};
use specs::prelude::*;
use std::{sync::mpsc::Sender, time::Duration};

#[derive(SystemData)]
pub struct ScriptSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,

    ids: ReadStorage<'a, Id>,
    scripts: WriteStorage<'a, ScriptState>,
    positions: WriteStorage<'a, Position>,
    sprites: WriteStorage<'a, Sprite>,
}

pub struct ScriptSystem {
    tx: Sender<Action>,
}

impl<'a> System<'a> for ScriptSystem {
    type SystemData = ScriptSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for (entity, id, script, position, sprite) in (
            &data.entities,
            &data.ids,
            &mut data.scripts,
            &mut data.positions,
            &mut data.sprites,
        )
            .join()
        {
            let sprite_sheet = &data.sheets_manager.load(&sprite.resource).unwrap();
            let mut animated = Animated::new(entity, position, sprite, sprite_sheet);

            if script.runner.state() == AnimationRunningState::Init {
                script.runner.start(&mut animated, &script.script);
            }

            if script.runner.state() == AnimationRunningState::Running {
                script
                    .runner
                    .progress(*data.time_since_last_frame, &mut animated, &script.script);
            }

            if script.runner.state() == AnimationRunningState::Finished {
                self.emit_done(id, script, position, sprite);
                data.updater.remove::<ScriptState>(entity);
            }
        }
    }
}

impl ScriptSystem {
    pub fn new(tx: Sender<Action>) -> Self {
        ScriptSystem { tx }
    }

    fn emit_done(&self, id: &Id, script: &ScriptState, position: &Position, sprite: &Sprite) {
        self.tx
            .send(Action {
                action: Some(action::Action::Emit(EmitAction {
                    event: Some(Event {
                        event_id: format!("{}_script_done", id.0),
                        event: Some(event::Event::AnimationScriptDone(AnimationEvent {
                            animation_id: script.script.id.clone(),
                            position: Some(Vector {
                                x: position.0.x() as f64,
                                y: position.0.y() as f64,
                                z: 0.0,
                            }),
                            frame_index: sprite.frame_index as u32,
                        })),
                    }),
                })),
            })
            .expect("🦀 Action channel closed.");
    }
}
