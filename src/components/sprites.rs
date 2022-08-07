use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Dirty;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Id(pub String);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Rect);

impl Default for Position {
    fn default() -> Self {
        Position(Rect::new(0, 0, 0, 0))
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteInfo {
    pub texture_id: String,
    pub frame_index: usize,
    pub bounding_box: Rect,
}
