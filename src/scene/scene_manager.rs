use super::{scene::Scene, scene_builder::SceneBuilder};
use crate::{
    core::Status,
    resources::TextureManager,
    resources::{SpriteManager, TileMapManager},
};
use sdl2::{rect::Rect, render::WindowCanvas};
use specs::prelude::*;

pub struct SceneManager {
    scene: Scene,
    viewport: Rect,
    window_size: Rect,
    tilemap_manager: TileMapManager,
    tile_sprite_manager: SpriteManager,
}

impl SceneManager {
    pub fn new(resource_path: &str, window_width: u32, window_height: u32) -> Self {
        SceneManager {
            scene: Scene {
                layers: vec![],
                bounds: Rect::new(0, 0, window_width, window_height),
            },
            viewport: Rect::new(0, 0, window_width, window_height),
            window_size: Rect::new(0, 0, window_width, window_height),
            tilemap_manager: TileMapManager::create(resource_path),
            tile_sprite_manager: SpriteManager::create(resource_path),
        }
    }

    pub fn load(
        &mut self,
        resource: &str,
        viewport: Option<Rect>,
        world: &mut World,
    ) -> Result<(), Status> {
        if let Some(viewport) = viewport {
            if viewport.x() < 0
                || viewport.y() < 0
                || viewport.x() as u32 + viewport.width() > self.window_size.width()
                || viewport.y() as u32 + viewport.height() > self.window_size.height()
            {
                return Err(Status::invalid_argument(&format!(
                    "viewport should not have negative coordinates and its size should fit the window: {:?}",
                    viewport
                )));
            }
            self.viewport = viewport;
        }

        self.tilemap_manager.load(resource)?;
        let map = self.tilemap_manager.get(resource).unwrap();

        for set in &map.tilesets {
            self.tile_sprite_manager.load(
                &set.source
                    .strip_suffix(".tsx")
                    .expect("TileSet does not have the expected format."),
            )?;
        }

        self.scene = SceneBuilder::build(map, &self.tile_sprite_manager, world);
        println!("🦀 scene '{resource}' loaded");

        Ok(())
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    ) -> Result<(), Status> {
        for layer in &self.scene.layers {
            for tile in &layer.tiles {
                let texture = texture_manager.load(&tile.texture_id).unwrap();
                canvas.copy(&texture, tile.texture_position, tile.canvas_position)?;
            }
        }

        Ok(())
    }
}
