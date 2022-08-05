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
    tilemap_manager: TileMapManager,
    tile_sprite_manager: SpriteManager,
}

impl SceneManager {
    pub fn new(resource_path: &str) -> Self {
        SceneManager {
            scene: Scene {
                layers: vec![],
                bounds: Rect::new(0, 0, 0, 0),
            },
            tilemap_manager: TileMapManager::create(resource_path),
            tile_sprite_manager: SpriteManager::create(resource_path),
        }
    }

    pub fn scene_bounds(&self) -> Rect {
        self.scene.bounds
    }

    pub fn load(&mut self, resource: &str, world: &mut World) -> Result<(), Status> {
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
        viewport: Rect,
        canvas: &mut WindowCanvas,
        texture_manager: &mut TextureManager<sdl2::video::WindowContext>,
    ) -> Result<(), Status> {
        for layer in &self.scene.layers {
            for tile in &layer.tiles {
                let texture = texture_manager.load(&tile.texture_id).unwrap();
                canvas.copy(
                    &texture,
                    tile.texture_position,
                    Rect::new(
                        tile.canvas_position.x() - viewport.x(),
                        tile.canvas_position.y() - viewport.y(),
                        tile.canvas_position.width(),
                        tile.canvas_position.height(),
                    ),
                )?;
            }
        }

        Ok(())
    }
}
