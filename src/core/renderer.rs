use crate::components::{Position, Sprite};
use crate::sdl::TextureManager;
use crate::Status;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::prelude::*;

type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (pos, sprite): SystemData,
) -> Result<(), Status> {
    // canvas.set_draw_color(background);
    canvas.clear();

    for (pos, sprite) in (&pos, &sprite).join() {
        let texture = texture_manager.load(&sprite.resource)?;
        canvas.copy(
            &texture,
            sprite.bounding_box,
            Rect::new(
                pos.0.x(),
                pos.0.y(),
                2 * sprite.bounding_box.width(),
                2 * sprite.bounding_box.height(),
            ),
        )?;
    }

    canvas.present();

    Ok(())
}
