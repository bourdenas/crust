use super::scene_manager::Scene;
use super::SceneManager;
use crate::components::{Position, Size, SpriteInfo};
use crate::core::Status;
use crate::resources::TextureManager;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::prelude::*;

type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, SpriteInfo>,
    ReadStorage<'a, Size>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    scene_manager: &SceneManager,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    (positions, sprite_info, sizes): SystemData,
) -> Result<(), Status> {
    // canvas.set_draw_color(background);
    canvas.clear();

    render_scene(canvas, &scene_manager.scene, texture_manager)?;

    for (position, sprite_info, size) in (&positions, &sprite_info, &sizes).join() {
        let texture = texture_manager.load(&sprite_info.texture_id)?;

        canvas.copy(
            &texture,
            size.bounding_box,
            Rect::new(
                position.0.x(),
                position.0.y(),
                (size.bounding_box.width() as f64 * size.scaling.x) as u32,
                (size.bounding_box.height() as f64 * size.scaling.y) as u32,
            ),
        )?;
    }

    canvas.present();

    Ok(())
}

fn render_scene(
    canvas: &mut WindowCanvas,
    scene: &Scene,
    texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
) -> Result<(), Status> {
    for layer in &scene.layers {
        for tile in &layer.tiles {
            let texture = texture_manager.load(&tile.texture_id).unwrap();
            canvas.copy(&texture, tile.texture_position, tile.canvas_position)?;
        }
    }

    Ok(())
}
