use super::INDEX;
use crate::components::Animation;
use crate::crust::{AnimationScriptAction, SceneNodeRefAction};
use specs::prelude::*;

pub struct Animations;

impl Animations {
    pub fn play(script_action: AnimationScriptAction, world: &mut World) {
        if let Some(script) = script_action.script {
            let mut entity_id = None;
            INDEX.with(|index| {
                if let Some(index) = &*index.borrow() {
                    entity_id = index.find_entity(&script_action.scene_node_id);
                }
            });

            if let Some(id) = entity_id {
                let entity = world.entities().entity(id);

                let mut scripts = world.write_storage::<Animation>();
                if let Err(e) = scripts.insert(entity, Animation::new(script)) {
                    eprintln!("play_animation(): {}", e);
                }
            }
        }
    }

    pub fn stop(scene_node_ref_action: SceneNodeRefAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &*index.borrow() {
                entity_id = index.find_entity(&scene_node_ref_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);

            let mut scripts = world.write_storage::<Animation>();
            scripts.remove(entity);
        }
    }
}
