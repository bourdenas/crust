use sdl2::rect::Point;
use specs::prelude::*;
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ScrollingInfo {
    pub direction: Point,
    pub wait_time: Duration,
}
