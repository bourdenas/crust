use super::Status;
use crate::resources::{SpriteSheetsManager, TileMap, TileMapManager};
use sdl2::rect::Rect;
use specs::World;

#[derive(Default)]
pub struct Scene {
    pub layers: Vec<SceneLayer>,
}
#[derive(Default)]
pub struct SceneLayer {
    pub tiles: Vec<TileInfo>,
}

pub struct TileInfo {
    pub texture_id: String,
    pub texture_position: Rect,
    pub canvas_position: Rect,
}

pub struct SceneManager {
    pub scene: Scene,
    tilemap_manager: TileMapManager,
    tile_sheet_manager: SpriteSheetsManager,
}

impl SceneManager {
    pub fn new(resource_path: &str) -> Self {
        SceneManager {
            scene: Scene::default(),
            tilemap_manager: TileMapManager::create(resource_path),
            tile_sheet_manager: SpriteSheetsManager::create(resource_path),
        }
    }

    pub fn load(&mut self, resource: &str, world: &mut World) -> Result<(), Status> {
        self.tilemap_manager.load(resource)?;
        let map = self.tilemap_manager.get(resource).unwrap();

        for set in &map.tilesets {
            self.tile_sheet_manager.load(
                &set.source
                    .strip_suffix(".tsx")
                    .expect("TileSet does not have the expected format."),
            )?;
        }

        self.scene = self.build_scene(map);
        println!("🦀 scene '{resource}' loaded");

        Ok(())
    }

    fn build_scene(&self, map: &TileMap) -> Scene {
        let ranges = Self::build_tileset_ranges(map);

        let mut scene = Scene::default();
        for layer in &map.layers {
            if layer.layer_type != "tilelayer" {
                continue;
            }

            scene.layers.push(SceneLayer {
                tiles: layer
                    .data
                    .iter()
                    .enumerate()
                    .filter(|(_, tile_id)| **tile_id > 0)
                    .map(|(i, tile_id)| {
                        let range = ranges
                            .iter()
                            .find(|range| range.first <= *tile_id && *tile_id < range.last)
                            .expect(&format!(
                                "Failed to find {tile_id} in ranges: {:?}",
                                &ranges
                            ));
                        TileInfo {
                            texture_id: range.resource.clone(),
                            texture_position: self
                                .tile_sheet_manager
                                .get_box(&range.resource, (tile_id - range.first) as usize)
                                .expect(&format!(
                                    "Tile index '{tile_id}' exceeds available tiles in {}",
                                    &range.resource
                                )),
                            canvas_position: Rect::new(
                                ((i as u32 % map.width) * map.tilewidth) as i32,
                                ((i as u32 / map.width) * map.tileheight) as i32,
                                map.tilewidth,
                                map.tileheight,
                            ),
                        }
                    })
                    .collect(),
            });
        }
        scene
    }

    fn build_tileset_ranges(map: &TileMap) -> Vec<Range> {
        let tilesets = &map.tilesets;

        let mut ranges = vec![];
        for i in 0..tilesets.len() {
            ranges.push(Range {
                first: tilesets[i].firstgid,
                last: match i + 1 < tilesets.len() {
                    true => tilesets[i + 1].firstgid - 1,
                    false => u32::MAX,
                },
                resource: tilesets[i].source.strip_suffix(".tsx").unwrap().to_owned(),
            });
        }

        ranges
    }
}

#[derive(Debug)]
struct Range {
    first: u32,
    last: u32,
    resource: String,
}
