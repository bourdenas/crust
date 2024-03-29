use crate::{
    action::ActionQueue,
    components::{Id, Position, Rotation, Scaling, SpriteInfo, Velocity},
    crust::{HorizontalAlign, VerticalAlign},
    resources::Sprite,
};
use sdl2::rect::Point;

pub struct Animated<'a> {
    pub id: &'a Id,
    pub position: &'a mut Position,
    pub velocity: &'a mut Velocity,
    pub rotation: &'a mut Rotation,
    pub scaling: &'a mut Scaling,
    pub sprite_info: &'a mut SpriteInfo,
    sprite: &'a Sprite,
    pub queue: Option<&'a ActionQueue>,
}

impl<'a> Animated<'a> {
    pub fn new(
        id: &'a Id,
        position: &'a mut Position,
        velocity: &'a mut Velocity,
        rotation: &'a mut Rotation,
        scaling: &'a mut Scaling,
        sprite_info: &'a mut SpriteInfo,
        sprite: &'a Sprite,
        queue: Option<&'a ActionQueue>,
    ) -> Self {
        Animated {
            id,
            position,
            velocity,
            rotation,
            scaling,
            sprite_info,
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
        let prev_aabb = self.position.0;
        let mut next_aabb = self.sprite.frames[frame_index].bounding_box;
        next_aabb.reposition(self.position.0.top_left());
        next_aabb.resize(
            (next_aabb.width() as f64 * self.scaling.0 .0) as u32,
            (next_aabb.height() as f64 * self.scaling.0 .1) as u32,
        );

        self.position.0 = next_aabb;
        self.sprite_info.frame_index = frame_index;
        self.sprite_info.bounding_box = self.sprite.frames[frame_index].bounding_box;

        self.velocity.0 += Point::new(
            match h_align {
                HorizontalAlign::Right => prev_aabb.width() as i32 - next_aabb.width() as i32,
                HorizontalAlign::Hcentre => {
                    (prev_aabb.width() as i32 - next_aabb.width() as i32) / 2
                }
                _ => 0,
            },
            match v_align {
                VerticalAlign::Bottom => prev_aabb.height() as i32 - next_aabb.height() as i32,
                VerticalAlign::Vcentre => {
                    (prev_aabb.height() as i32 - next_aabb.height() as i32) / 2
                }
                _ => 0,
            },
        );
    }
}
