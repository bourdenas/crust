use crate::{
    action::ActionQueue,
    animation::Animated,
    components::{Animation, AnimationRunningState, Id, Position, Size, Sprite, Velocity},
    crust::{event, AnimationEvent, Vector},
    resources::SpriteSheetsManager,
};
use specs::prelude::*;
use std::time::Duration;

#[derive(SystemData)]
pub struct AnimatorSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
    sheets_manager: WriteExpect<'a, SpriteSheetsManager>,
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,

    ids: ReadStorage<'a, Id>,
    animations: WriteStorage<'a, Animation>,
    positions: ReadStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
    sprites: WriteStorage<'a, Sprite>,
    sizes: WriteStorage<'a, Size>,
}

pub struct AnimatorSystem {
    queue: ActionQueue,
}

impl<'a> System<'a> for AnimatorSystem {
    type SystemData = AnimatorSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for (entity, id, animation, position, velocity, sprite, size) in (
            &data.entities,
            &data.ids,
            &mut data.animations,
            &data.positions,
            &mut data.velocities,
            &mut data.sprites,
            &mut data.sizes,
        )
            .join()
        {
            let sprite_sheet = &data.sheets_manager.load(&sprite.resource).unwrap();
            let mut animated = Animated::new(
                id,
                position,
                velocity,
                sprite,
                size,
                sprite_sheet,
                Some(&self.queue),
            );

            if animation.runner.state() == AnimationRunningState::Init {
                animation.runner.start(&mut animated);
            }

            if animation.runner.state() == AnimationRunningState::Running {
                animation
                    .runner
                    .progress(*data.time_since_last_frame, &mut animated);
            }

            if animation.runner.state() == AnimationRunningState::Finished {
                self.emit_done(id, animation, position, sprite);
                data.updater.remove::<Animation>(entity);
            }
        }
    }
}

impl AnimatorSystem {
    pub fn new(queue: ActionQueue) -> Self {
        AnimatorSystem { queue }
    }

    fn emit_done(&self, id: &Id, script: &Animation, position: &Position, sprite: &Sprite) {
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
