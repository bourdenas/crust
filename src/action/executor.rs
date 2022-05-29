use super::INDEX;
use crate::components::{Id, Position, ScriptState, Sprite};
use crate::crust::{
    action, Action, AnimationScriptAction, SceneNodeAction, SceneNodeRefAction, Vector,
};
use sdl2::rect::Point;
use specs::prelude::*;

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn new() -> Self {
        ActionExecutor {}
    }

    pub fn execute(&self, action: Action, world: &mut World) {
        match action.action {
            Some(action::Action::Quit(..)) => (),
            Some(action::Action::CreateSceneNode(action)) => self.create_scene_node(action, world),
            Some(action::Action::DestroySceneNode(action)) => {
                self.destroy_scene_node(action, world)
            }
            Some(action::Action::PlayAnimation(action)) => self.play_animation(action, world),
            Some(action::Action::StopAnimation(action)) => self.stop_animation(action, world),
            _ => (),
        }
    }

    fn create_scene_node(&self, scene_node_action: SceneNodeAction, world: &mut World) {
        if let Some(node) = scene_node_action.scene_node {
            let entity = world
                .create_entity()
                .with(Id(node.id.clone()))
                .with(Position(make_point(
                    &node.position.expect("Node missing position"),
                )))
                .with(Sprite {
                    resource: node.sprite_id,
                    frame_index: node.frame_index as usize,
                })
                .build();

            INDEX.with(|index| {
                if let Some(index) = &mut *index.borrow_mut() {
                    index.add_entity(&node.id, entity.id());
                }
            });
        }
    }

    fn destroy_scene_node(&self, scene_node_action: SceneNodeRefAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &mut *index.borrow_mut() {
                entity_id = index.remove_entity(&scene_node_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);
            if let Err(e) = world.delete_entity(entity) {
                eprintln!("destroy_scene_node(): {}", e);
            }
        }
    }

    fn play_animation(&self, script_action: AnimationScriptAction, world: &mut World) {
        if let Some(script) = script_action.script {
            let mut entity_id = None;
            INDEX.with(|index| {
                if let Some(index) = &*index.borrow() {
                    entity_id = index.find_entity(&script_action.scene_node_id);
                }
            });

            if let Some(id) = entity_id {
                let entity = world.entities().entity(id);

                let mut scripts = world.write_storage::<ScriptState>();
                if let Err(e) = scripts.insert(entity, ScriptState::new(script)) {
                    println!("play_animation(): {}", e);
                }
            }
        }
    }

    fn stop_animation(&self, scene_node_ref_action: SceneNodeRefAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &*index.borrow() {
                entity_id = index.find_entity(&scene_node_ref_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);

            let mut scripts = world.write_storage::<ScriptState>();
            scripts.remove(entity);
        }
    }
}

fn make_point(vec: &Vector) -> Point {
    Point::new(vec.x as i32, vec.y as i32)
}
