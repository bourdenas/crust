use crate::{
    crust::SceneAction,
    resources::{Viewport, WindowSize},
    scene::SceneManager,
};
use sdl2::rect::Rect;
use specs::{World, WorldExt};

pub struct Scenes;

impl Scenes {
    pub fn load(scene_action: SceneAction, scene_manager: &mut SceneManager, world: &mut World) {
        if let Err(e) = scene_manager.load(&scene_action.resource, None, world) {
            eprintln!("🦀 Failed to load scene: {:?}\nError: {e}", &scene_action);
            return;
        }

        match &scene_action.viewport {
            Some(viewport) => {
                *world.write_resource() = Viewport(Rect::new(
                    viewport.left,
                    viewport.top,
                    viewport.width,
                    viewport.height,
                ))
            }
            None => *world.write_resource() = Viewport(world.read_resource::<WindowSize>().0),
        }
    }
}
