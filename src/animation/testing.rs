#[cfg(test)]
pub mod util {
    use crate::{
        animation::Animated,
        components::{Id, Position, Rotation, Scaling, SpriteInfo, Velocity},
        resources::{Frame, Sprite},
    };
    use sdl2::rect::Rect;

    pub struct Fixture {
        id: Id,
        pub position: Position,
        pub velocity: Velocity,
        pub rotation: Rotation,
        pub scaling: Scaling,
        pub sprite_info: SpriteInfo,
        sprite: Sprite,
    }

    impl Fixture {
        pub fn new() -> Self {
            Fixture {
                id: Id("test_id".to_owned()),
                position: Position(Rect::new(0, 0, 32, 32)),
                velocity: Velocity::default(),
                rotation: Rotation::default(),
                scaling: Scaling::default(),
                sprite_info: SpriteInfo {
                    texture_id: "foo".to_owned(),
                    frame_index: 0,
                    bounding_box: Rect::new(0, 0, 32, 32),
                },
                sprite: Sprite {
                    texture_id: "foo".to_owned(),
                    frames: vec![
                        Frame {
                            bounding_box: Rect::new(0, 0, 32, 32),
                            bitmask: None,
                        },
                        Frame {
                            bounding_box: Rect::new(0, 0, 32, 30),
                            bitmask: None,
                        },
                        Frame {
                            bounding_box: Rect::new(0, 0, 32, 32),
                            bitmask: None,
                        },
                        Frame {
                            bounding_box: Rect::new(0, 0, 32, 32),
                            bitmask: None,
                        },
                        Frame {
                            bounding_box: Rect::new(0, 0, 28, 32),
                            bitmask: None,
                        },
                        Frame {
                            bounding_box: Rect::new(0, 0, 32, 32),
                            bitmask: None,
                        },
                    ],
                },
            }
        }

        pub fn animated(&mut self) -> Animated {
            Animated::new(
                &self.id,
                &mut self.position,
                &mut self.velocity,
                &mut self.rotation,
                &mut self.scaling,
                &mut self.sprite_info,
                &self.sprite,
                None,
            )
        }
    }
}
