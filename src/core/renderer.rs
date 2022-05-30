use crate::components::{Position, Sprite};
use crate::core::{Status, TextureManager};
use crate::resources::SpriteSheetsManager;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::prelude::*;

type SystemData<'a> = (
    WriteExpect<'a, SpriteSheetsManager>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (mut sheets_manager, pos, sprite): SystemData,
) -> Result<(), Status> {
    // canvas.set_draw_color(background);
    canvas.clear();

    for (pos, sprite) in (&pos, &sprite).join() {
        let texture = texture_manager.load(&sprite.resource)?;
        let sprite_sheet = sheets_manager.load(&sprite.resource)?;

        if sprite_sheet.bounding_boxes.len() <= sprite.frame_index {
            continue;
        }

        let bb = sprite_sheet.bounding_boxes[sprite.frame_index];
        canvas.copy(
            &texture,
            bb,
            Rect::new(
                pos.0.x(),
                pos.0.y(),
                (bb.width() as f64 * sprite.scaling.x) as u32,
                (bb.height() as f64 * sprite.scaling.x) as u32,
            ),
        )?;
    }

    canvas.present();

    Ok(())
}
