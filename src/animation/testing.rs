#[cfg(test)]
pub mod util {
    use crate::{
        animation::Animated,
        components::{Id, Position, ScalingVec, Size, SpriteInfo, Velocity},
        resources::Sprite,
    };
    use sdl2::rect::{Point, Rect};

    pub struct Fixture {
        id: Id,
        pub position: Position,
        pub velocity: Velocity,
        pub sprite_info: SpriteInfo,
        pub size: Size,
        sprite: Sprite,
    }

    impl Fixture {
        pub fn new() -> Self {
            Fixture {
                id: Id("test_id".to_owned()),
                position: Position(Point::new(0, 0)),
                velocity: Velocity(Point::new(0, 0)),
                sprite_info: SpriteInfo {
                    texture_id: "foo".to_owned(),
                    frame_index: 0,
                },
                size: Size {
                    bounding_box: Rect::new(0, 0, 32, 32),
                    scaling: ScalingVec::default(),
                },
                sprite: Sprite {
                    texture_id: "foo".to_owned(),
                    frames: vec![
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
                &self.id,
                &mut self.position,
                &mut self.velocity,
                &mut self.sprite_info,
                &mut self.size,
                &self.sprite,
                None,
            )
        }
    }
}
