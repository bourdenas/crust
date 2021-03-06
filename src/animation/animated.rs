use crate::{
    action::ActionQueue,
    components::{Id, Position, Size, SpriteInfo, Velocity},
    crust::{HorizontalAlign, VerticalAlign},
    resources::Sprite,
};
use sdl2::rect::Point;

pub struct Animated<'a> {
    pub id: &'a Id,
    pub position: &'a Position,
    pub velocity: &'a mut Velocity,
    pub sprite_info: &'a mut SpriteInfo,
    pub size: &'a mut Size,
    sprite: &'a Sprite,
    pub queue: Option<&'a ActionQueue>,
}

impl<'a> Animated<'a> {
    pub fn new(
        id: &'a Id,
        position: &'a Position,
        velocity: &'a mut Velocity,
        sprite_info: &'a mut SpriteInfo,
        size: &'a mut Size,
        sprite: &'a Sprite,
        queue: Option<&'a ActionQueue>,
    ) -> Self {
        Animated {
            id,
            position,
            velocity,
            sprite_info,
            size,
            sprite,
            queue,
        }
    }

    /// Handles sprite frame changes taking care of sprite film alignments.
    pub fn change_frame(
        &mut self,
        frame_index: usize,
        v_align: VerticalAlign,
        h_align: HorizontalAlign,
    ) {
        let mut prev_aabb = self.sprite.frames[self.sprite_info.frame_index];
        prev_aabb.reposition(self.position.0);
        let mut next_aabb = self.sprite.frames[frame_index];
        next_aabb.reposition(self.position.0);

        self.sprite_info.frame_index = frame_index;
        self.size.bounding_box = self.sprite.frames[frame_index];

        self.velocity.0 += Point::new(
            match h_align {
                HorizontalAlign::Right => {
                    self.position.0.x() + (prev_aabb.width() - next_aabb.width()) as i32
                }
                HorizontalAlign::Hcentre => {
                    self.position.0.x() + ((prev_aabb.width() - next_aabb.width()) / 2) as i32
                }
                _ => 0,
            },
            match v_align {
                VerticalAlign::Bottom => {
                    self.position.0.y() + (prev_aabb.height() - next_aabb.height()) as i32
                }
                VerticalAlign::Vcentre => {
                    self.position.0.y() + (prev_aabb.height() - next_aabb.height() / 2) as i32
                }
                _ => 0,
            },
        );
    }
}
