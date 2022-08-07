use crate::{components::ScrollingInfo, resources::Viewport};
use specs::prelude::*;
use std::time::Duration;

#[derive(SystemData)]
pub struct ScrollingSystemData<'a> {
    time_since_last_frame: ReadExpect<'a, Duration>,
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
