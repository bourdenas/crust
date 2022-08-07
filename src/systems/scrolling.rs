use crate::{
    components::ScrollingInfo,
    resources::{Viewport, WorldSize},
};
use sdl2::rect::Point;
use specs::prelude::*;
use std::time::Duration;

#[derive(SystemData)]
pub struct ScrollingSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
    world_size: WriteExpect<'a, WorldSize>,
    viewport: WriteExpect<'a, Viewport>,

    scrolling_info: WriteStorage<'a, ScrollingInfo>,
}

pub struct ScrollingSystem;

impl<'a> System<'a> for ScrollingSystem {
    type SystemData = ScrollingSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut data = data;

        for scrolling in (&mut data.scrolling_info).join() {
            scrolling.wait_time += *data.time_since_last_frame;
            while SCROLL_DELAY <= scrolling.wait_time {
                scrolling.wait_time -= SCROLL_DELAY;
                data.viewport
                    .0
                    .offset(scrolling.direction.x(), scrolling.direction.y());

                let pos = Point::new(
                    data.viewport.0.x().clamp(
                        0,
                        (data.world_size.0.width() - data.viewport.0.width()) as i32,
                    ),
                    data.viewport.0.y().clamp(
                        0,
                        (data.world_size.0.height() - data.viewport.0.height()) as i32,
                    ),
                );
                data.viewport.0.reposition(pos);
            }
        }
    }
}

impl ScrollingSystem {
    pub fn new() -> Self {
        ScrollingSystem {}
    }
}

const SCROLL_DELAY: Duration = Duration::from_millis(10);
