use specs::World;

use crate::{crust::SceneAction, scene::SceneManager};

pub struct Scenes;

impl Scenes {
    pub fn load(scene_action: SceneAction, scene_manager: &mut SceneManager, world: &mut World) {
        if let Err(e) = scene_manager.load(&scene_action.resource, world) {
            eprintln!("🦀 Failed to load scene: {:?}\nError: {e}", &scene_action);
        }
    }
}
