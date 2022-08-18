use crate::{
    components::{Position, Rotation, SpriteInfo},
    core::Status,
    crust::DebugInfo,
    resources::{TextureManager, Viewport},
    scene::SceneManager,
};
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};
use specs::prelude::*;

type SystemData<'a> = (
    ReadExpect<'a, DebugInfo>,
    ReadExpect<'a, Viewport>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Rotation>,
    ReadStorage<'a, SpriteInfo>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    scene_manager: &SceneManager,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (debug, viewport, positions, rotations, sprite_info): SystemData,
) -> Result<(), Status> {
    canvas.clear();

    scene_manager.render(viewport.0, canvas, texture_manager)?;

    for (position, rotation, sprite_info) in (&positions, &rotations, &sprite_info).join() {
        let texture = texture_manager.load(&sprite_info.texture_id)?;

        canvas.copy_ex(
            &texture,
            sprite_info.bounding_box,
            Rect::new(
                position.0.x() - viewport.0.x(),
                position.0.y() - viewport.0.y(),
                position.0.width(),
                position.0.height(),
            ),
            rotation.angle,
            rotation.centre,
            false,
            false,
        )?;
    }

    if debug.draw_bounding_boxes {
        for position in (&positions).join() {
            canvas.set_draw_color(Color::MAGENTA);
            canvas.draw_rect(Rect::new(
                position.0.x() - viewport.0.x(),
                position.0.y() - viewport.0.y(),
                position.0.width(),
                position.0.height(),
            ))?;
        }
    }

    canvas.present();

    Ok(())
}
