use super::INDEX;
use crate::components::{
    Animation, Collisions, Id, Position, RigidBody, ScalingVec, Sprite, Velocity,
};
use crate::crust::{
    action, Action, AnimationScriptAction, CollisionAction, EmitAction, SceneNodeAction,
    SceneNodeRefAction, Vector,
};
use crate::event::EventManager;
use crate::resources::SpriteSheetsManager;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use std::sync::mpsc::Receiver;

pub struct ActionExecutor {
    rx: Receiver<Action>,
}

impl ActionExecutor {
    pub fn new(rx: Receiver<Action>) -> Self {
        ActionExecutor { rx }
    }

    pub fn process(&self, world: &mut World, event_manager: &mut EventManager) {
        self.rx
            .try_iter()
            .for_each(|action| Self::execute(action, world, event_manager));
    }

    fn execute(action: Action, world: &mut World, event_manager: &mut EventManager) {
        match action.action {
            Some(action::Action::Quit(..)) => (),
            Some(action::Action::CreateSceneNode(action)) => Self::create_scene_node(action, world),
            Some(action::Action::DestroySceneNode(action)) => {
                Self::destroy_scene_node(action, world)
            }
            Some(action::Action::PlayAnimation(action)) => Self::play_animation(action, world),
            Some(action::Action::StopAnimation(action)) => Self::stop_animation(action, world),
            Some(action::Action::OnCollision(action)) => Self::on_collision(action, world),
            Some(action::Action::Emit(action)) => Self::emit(action, event_manager),
            _ => (),
        }
    }

    fn get_bounding_box(world: &mut World, resource: &str, frame_index: usize) -> Option<Rect> {
        let mut sheets_manager = world.write_resource::<SpriteSheetsManager>();
        if let Err(e) = sheets_manager.load(resource) {
            eprintln!("🦀 {}", e);
            return None;
        }

        sheets_manager.get_box(resource, frame_index)
    }

    fn create_scene_node(scene_node_action: SceneNodeAction, world: &mut World) {
        if let Some(node) = scene_node_action.scene_node {
            let bbox =
                match Self::get_bounding_box(world, &node.sprite_id, node.frame_index as usize) {
                    Some(bbox) => bbox,
                    None => {
                        eprintln!(
                            "🦀 Failed to retrieve frame '{}' from resouce sheet '{}'",
                            node.frame_index, &node.sprite_id
                        );
                        return;
                    }
                };

            let mut builder = world
                .create_entity()
                .with(Id(node.id.clone()))
                .with(Position(make_point(
                    &node.position.expect("Node missing position"),
                )))
                .with(Velocity(Point::new(0, 0)))
                .with(Sprite {
                    resource: node.sprite_id.clone(),
                    frame_index: node.frame_index as usize,
                    bounding_box: bbox,
                    scaling: ScalingVec::default(),
                });

            if node.rigid_body {
                builder = builder.with(RigidBody {});
            }
            let entity = builder.build();

            INDEX.with(|index| {
                if let Some(index) = &mut *index.borrow_mut() {
                    index.add_entity(&node.id, entity.id());
                }
            });
        }
    }

    fn destroy_scene_node(scene_node_action: SceneNodeRefAction, world: &mut World) {
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

    fn play_animation(script_action: AnimationScriptAction, world: &mut World) {
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

    fn stop_animation(scene_node_ref_action: SceneNodeRefAction, world: &mut World) {
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

    fn on_collision(collision_action: CollisionAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &*index.borrow() {
                entity_id = index.find_entity(&collision_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);

            let mut collisions = world.write_storage::<Collisions>();
            match collisions.get_mut(entity) {
                Some(collisions) => {
                    collisions.on_collision.push(collision_action);
                }
                None => {
                    if let Err(e) = collisions.insert(
                        entity,
                        Collisions {
                            on_collision: vec![collision_action],
                        },
                    ) {
                        eprintln!("on_collision(): {}", e);
                    }
                }
            }
        }
    }

    fn emit(emit_action: EmitAction, event_manager: &mut EventManager) {
        if let Some(event) = emit_action.event {
            event_manager.handle(event);
        }
    }
}

fn make_point(vec: &Vector) -> Point {
    Point::new(vec.x as i32, vec.y as i32)
}
