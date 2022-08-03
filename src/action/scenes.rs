use crate::{crust::SceneAction, scene::SceneManager};
use sdl2::rect::Rect;
use specs::World;

pub struct Scenes;

impl Scenes {
    pub fn load(scene_action: SceneAction, scene_manager: &mut SceneManager, world: &mut World) {
        if let Err(e) = scene_manager.load(
            &scene_action.resource,
            match &scene_action.viewport {
                Some(viewport) => Some(Rect::new(
                    viewport.left,
                    viewport.top,
                    viewport.width,
                    viewport.height,
                )),
                None => None,
            },
            world,
        ) {
            eprintln!("🦀 Failed to load scene: {:?}\nError: {e}", &scene_action);
        }
    }
}
