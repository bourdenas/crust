use crate::components::ScrollingInfo;
use crate::crust::ScrollAction;
use sdl2::rect::Point;
use specs::prelude::*;
use std::time::Duration;

pub struct Scrolling {
    viewport_entity: Entity,
}

impl Scrolling {
    pub fn new(world: &mut World) -> Self {
        Scrolling {
            viewport_entity: world
                .create_entity()
                .with(ScrollingInfo {
                    direction: Point::new(0, 0),
                    wait_time: Duration::ZERO,
                })
                .build(),
        }
    }

    pub fn scroll(&self, scroll_action: ScrollAction, world: &mut World) {
        if let Some(vec) = scroll_action.vec {
            let mut scrolling = world.write_storage::<ScrollingInfo>();
            if let Some(scroll_info) = scrolling.get_mut(self.viewport_entity) {
                scroll_info.direction += Point::new(vec.x as i32, vec.y as i32);
            }
        }
    }
}
