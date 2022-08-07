use crate::{
    action::ActionQueue,
    animation::Animated,
    components::{Animation, AnimationRunningState, Id, Position, Scaling, SpriteInfo, Velocity},
    crust::{event, AnimationEvent, Vector},
    resources::SpriteManager,
};
use specs::prelude::*;
use std::time::Duration;

#[derive(SystemData)]
pub struct AnimatorSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
    sprite_manager: WriteExpect<'a, SpriteManager>,
    entities: Entities<'a>,
    updater: Read<'a, LazyUpdate>,

    ids: ReadStorage<'a, Id>,
    animations: WriteStorage<'a, Animation>,
    positions: WriteStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
    scaling: WriteStorage<'a, Scaling>,
    sprite_info: WriteStorage<'a, SpriteInfo>,
}

pub struct AnimatorSystem {
    queue: ActionQueue,
}

impl<'a> System<'a> for AnimatorSystem {
    type SystemData = AnimatorSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for (entity, id, animation, position, velocity, scaling, sprite_info) in (
            &data.entities,
            &data.ids,
            &mut data.animations,
            &mut data.positions,
            &mut data.velocities,
            &mut data.scaling,
            &mut data.sprite_info,
        )
            .join()
        {
            let sprite = &data.sprite_manager.load(&sprite_info.texture_id).unwrap();
            let mut animated = Animated::new(
                id,
                position,
                velocity,
                scaling,
                sprite_info,
                sprite,
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
                self.emit_done(id, animation, position, sprite_info);
                data.updater.remove::<Animation>(entity);
            }
        }
    }
}

impl AnimatorSystem {
    pub fn new(queue: ActionQueue) -> Self {
        AnimatorSystem { queue }
    }

    fn emit_done(
        &self,
        id: &Id,
        script: &Animation,
        position: &Position,
        sprite_info: &SpriteInfo,
    ) {
        self.queue.emit(
            format!("{}_script_done", id.0),
            event::Event::AnimationScriptDone(AnimationEvent {
                animation_id: script.runner.script.id.clone(),
                position: Some(Vector {
                    x: position.0.x() as f64,
                    y: position.0.y() as f64,
                    z: 0.0,
                }),
                frame_index: sprite_info.frame_index as u32,
            }),
        );
    }
}
