use sdl2::rect::Point;
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
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Position(Point::new(0, 0))
    }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub resource: String,
    pub frame_index: usize,
}
