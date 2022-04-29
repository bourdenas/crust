use sdl2::rect::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub resource: String,
    pub frame_index: usize,
}
