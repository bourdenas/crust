use crate::{
    crust::SceneAction,
    resources::{Viewport, WindowSize, WorldSize},
    scene::SceneManager,
};
use sdl2::rect::Rect;
use specs::{World, WorldExt};

pub struct Scenes;

impl Scenes {
    pub fn load(scene_action: SceneAction, scene_manager: &mut SceneManager, world: &mut World) {
        if let Err(e) = scene_manager.load(&scene_action.resource, world) {
            eprintln!("🦀 Failed to load scene: {:?}\nError: {e}", &scene_action);
            return;
        }

        let window_size = world.read_resource::<WindowSize>().0;
        let scene_bounds = scene_manager.scene_bounds();

        *world.write_resource() = WorldSize(scene_bounds);
        *world.write_resource() = match &scene_action.viewport {
            Some(viewport) => {
                if viewport.left < 0
                    || viewport.top < 0
                    || viewport.left as u32 + viewport.width > scene_bounds.width()
                    || viewport.top as u32 + viewport.height > scene_bounds.height()
                    || viewport.width > window_size.width()
                    || viewport.height > window_size.height()
                {
                    *world.write_resource() = Viewport(window_size);
                    eprintln!(
                        "viewport {:?} should be fully included in the world bounds: {:?} and cannot be larger than the window size: {:?}",
                        &viewport, &scene_bounds, &window_size,
                    );
                }

                Viewport(Rect::new(
                    viewport.left,
                    viewport.top,
                    viewport.width,
                    viewport.height,
                ))
            }
            None => Viewport(window_size),
        }
    }
}
