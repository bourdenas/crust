use super::Animated;
use crate::{
    components::{Position, Sprite},
    resources::SpriteSheet,
};
use sdl2::rect::{Point, Rect};
use specs::prelude::*;

pub struct Fixture {
    world: World,
    pub position: Position,
    pub sprite: Sprite,
    sheet: SpriteSheet,
}

impl Fixture {
    pub fn new() -> Self {
        Fixture {
            world: World::new(),
            position: Position(Point::new(0, 0)),
            sprite: Sprite {
                resource: "foo".to_owned(),
                frame_index: 0,
            },
            sheet: SpriteSheet {
                resource: "foo".to_owned(),
                bounding_boxes: vec![
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                ],
            },
        }
    }

    pub fn animated(&mut self) -> Animated {
        Animated::new(
            self.world.create_entity().build(),
            &mut self.position,
            &mut self.sprite,
            &self.sheet,
        )
    }
}
