use super::scene::{Scene, SceneLayer, TileInfo};
use crate::{
    components::{Id, Position, RigidBody, ScalingVec, Size},
    resources::{ObjectProperty, SpriteManager, TileMap},
};
use sdl2::rect::Rect;
use specs::prelude::*;

pub struct SceneBuilder;

impl SceneBuilder {
    pub fn build(map: &TileMap, sprite_manager: &SpriteManager, world: &mut World) -> Scene {
        let ranges = Self::build_tileset_ranges(map);

        let mut layers = vec![];
        for layer in &map.layers {
            match layer.layer_type.as_str() {
                "tilelayer" => {
                    layers.push(SceneLayer {
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
                                    texture_position: sprite_manager
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
                "objectgroup" => {
                    for object in &layer.objects {
                        let mut builder = world
                            .create_entity()
                            .with(Id(object.name.clone()))
                            .with(Position(Rect::new(
                                object.x,
                                object.y,
                                object.width,
                                object.height,
                            )))
                            .with(Size {
                                bounding_box: Rect::new(0, 0, object.width, object.height),
                                scaling: ScalingVec::default(),
                            });

                        for property in &object.properties {
                            if let ObjectProperty::BoolType { name, value } = property {
                                if name == "rigid_body" && *value {
                                    builder = builder.with(RigidBody {});
                                }
                            }
                        }
                        builder.build();
                    }
                }
                _ => {}
            };
        }

        Scene {
            layers,
            bounds: Rect::new(0, 0, map.width * map.tilewidth, map.height * map.tileheight),
        }
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
