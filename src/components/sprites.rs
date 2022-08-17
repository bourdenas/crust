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

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Point::new(0, 0))
    }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    pub angle: f64,
    pub centre: Option<Point>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Scaling(pub (f64, f64));

impl Default for Scaling {
    fn default() -> Self {
        Scaling((1.0, 1.0))
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteInfo {
    pub texture_id: String,
    pub frame_index: usize,
    pub bounding_box: Rect,
}
