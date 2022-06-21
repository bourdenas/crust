#[cfg(test)]
pub mod util {
    use crate::{
        animation::Animated,
        components::{Id, Position, ScalingVec, Sprite, Velocity},
        resources::SpriteSheet,
    };
    use sdl2::rect::{Point, Rect};
    use specs::prelude::*;

    pub struct Fixture {
        world: World,
        id: Id,
        pub position: Position,
        velocity: Velocity,
        pub sprite: Sprite,
        sheet: SpriteSheet,
    }

    impl Fixture {
        pub fn new() -> Self {
            Fixture {
                world: World::new(),
                id: Id("test_id".to_owned()),
                position: Position(Point::new(0, 0)),
                velocity: Velocity(Point::new(0, 0)),
                sprite: Sprite {
                    resource: "foo".to_owned(),
                    frame_index: 0,
                    bounding_box: Rect::new(0, 0, 32, 32),
                    scaling: ScalingVec::default(),
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
                &mut self.velocity,
                &mut self.sprite,
                &self.sheet,
                None,
            )
        }
    }
}
