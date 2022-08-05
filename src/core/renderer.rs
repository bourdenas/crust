use crate::{
    components::{Position, SpriteInfo},
    core::Status,
    resources::{TextureManager, Viewport},
    scene::SceneManager,
};
use sdl2::render::WindowCanvas;
use specs::prelude::*;

type SystemData<'a> = (
    ReadExpect<'a, Viewport>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, SpriteInfo>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    scene_manager: &SceneManager,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (viewport, positions, sprite_info): SystemData,
) -> Result<(), Status> {
    // canvas.set_draw_color(background);
    canvas.clear();

    scene_manager.render(viewport.0, canvas, texture_manager)?;

    for (position, sprite_info) in (&positions, &sprite_info).join() {
        let texture = texture_manager.load(&sprite_info.texture_id)?;

        canvas.copy(&texture, sprite_info.bounding_box, position.0)?;
    }

    canvas.present();

    Ok(())
}
