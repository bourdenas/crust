use crate::components::{Position, Sprite};
use crate::core::Status;
use crate::resources::TextureManager;
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
                (sprite.bounding_box.width() as f64 * sprite.scaling.x) as u32,
                (sprite.bounding_box.height() as f64 * sprite.scaling.x) as u32,
            ),
        )?;
    }

    canvas.present();

    Ok(())
}
