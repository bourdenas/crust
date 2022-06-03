use super::Animated;
use crate::{
    components::{Id, Position, Sprite},
    resources::SpriteSheet,
};
use sdl2::rect::{Point, Rect};
use specs::prelude::*;

pub struct Fixture {
    world: World,
    id: Id,
    pub position: Position,
    pub sprite: Sprite,
    sheet: SpriteSheet,
}

impl Fixture {
    pub fn new() -> Self {
        Fixture {
            world: World::new(),
            id: Id("test_id".to_owned()),
            position: Position(Point::new(0, 0)),
            sprite: Sprite {
                resource: "foo".to_owned(),
                frame_index: 0,
                ..Default::default()
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
            &self.id,
            &mut self.position,
            &mut self.sprite,
            &self.sheet,
            None,
        )
    }
}
