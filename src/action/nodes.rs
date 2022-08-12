use super::INDEX;
use crate::components::{Id, Position, RigidBody, Rotation, Scaling, SpriteInfo, Velocity};
use crate::crust::{SceneNodeAction, SceneNodeRefAction, Vector};
use crate::resources::SpriteManager;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;

pub struct Nodes;

impl Nodes {
    pub fn create(node_action: SceneNodeAction, world: &mut World) {
        if let Some(node) = node_action.scene_node {
            let bbox = match frame_bounding_box(world, &node.sprite_id, node.frame_index as usize) {
                Some(bbox) => bbox,
                None => {
                    eprintln!(
                        "🦀 Failed to retrieve frame '{}' from resouce sheet '{}'",
                        node.frame_index, &node.sprite_id
                    );
                    return;
                }
            };

            let mut position = bbox;
            position.reposition(make_point(
                &node
                    .position
                    .expect(&format!("Node '{}' missing position", &node.id)),
            ));

            let mut builder = world
                .create_entity()
                .with(Id(node.id.clone()))
                .with(Position(position))
                .with(Velocity::default())
                .with(Rotation::default())
                .with(Scaling::default())
                .with(SpriteInfo {
                    texture_id: node.sprite_id.clone(),
                    frame_index: node.frame_index as usize,
                    bounding_box: bbox,
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

    pub fn destroy(node_ref_action: SceneNodeRefAction, world: &mut World) {
        let mut entity_id = None;
        INDEX.with(|index| {
            if let Some(index) = &mut *index.borrow_mut() {
                entity_id = index.remove_entity(&node_ref_action.scene_node_id);
            }
        });

        if let Some(id) = entity_id {
            let entity = world.entities().entity(id);
            if let Err(e) = world.delete_entity(entity) {
                eprintln!("🦀 destroy_scene_node(): {}", e);
            }
        }
    }
}

fn frame_bounding_box(world: &mut World, resource: &str, frame_index: usize) -> Option<Rect> {
    let mut sprite_manager = world.write_resource::<SpriteManager>();
    if let Err(e) = sprite_manager.load(resource) {
        eprintln!("🦀 {}", e);
        return None;
    }

    sprite_manager.get_box(resource, frame_index)
}

fn make_point(vec: &Vector) -> Point {
    Point::new(vec.x as i32, vec.y as i32)
}
