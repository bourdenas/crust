use crate::{
    action::ActionQueue,
    components::{Id, Position, Sprite, Velocity},
    crust::{HorizontalAlign, VerticalAlign},
    resources::SpriteSheet,
};
use sdl2::rect::Point;
use specs::Entity;

pub struct Animated<'a> {
    pub entity: Entity,
    pub id: &'a Id,
    pub position: &'a Position,
    pub velocity: &'a mut Velocity,
    pub sprite: &'a mut Sprite,
    pub sprite_sheet: &'a SpriteSheet,
    pub queue: Option<&'a ActionQueue>,
}

impl<'a> Animated<'a> {
    pub fn new(
        entity: Entity,
        id: &'a Id,
        position: &'a Position,
        velocity: &'a mut Velocity,
        sprite: &'a mut Sprite,
        sprite_sheet: &'a SpriteSheet,
        queue: Option<&'a ActionQueue>,
    ) -> Self {
        Animated {
            entity,
            id,
            position,
            velocity,
            sprite,
            sprite_sheet,
            queue,
        }
    }
}

/// Handles sprite frame changes taking care of sprite film alignments.
pub fn set_frame(
    frame_index: usize,
    v_align: VerticalAlign,
    h_align: HorizontalAlign,
    sprite: &mut Sprite,
    position: &Position,
    velocity: &mut Velocity,
    sprite_sheet: &SpriteSheet,
) {
    let mut prev_aabb = sprite_sheet.bounding_boxes[sprite.frame_index];
    prev_aabb.reposition(position.0);
    let mut next_aabb = sprite_sheet.bounding_boxes[frame_index];
    next_aabb.reposition(position.0);

    sprite.frame_index = frame_index;
    sprite.bounding_box = sprite_sheet.bounding_boxes[frame_index];

    velocity.0 += Point::new(
        match h_align {
            HorizontalAlign::Right => {
                position.0.x() + (prev_aabb.width() - next_aabb.width()) as i32
            }
            HorizontalAlign::Hcentre => {
                position.0.x() + ((prev_aabb.width() - next_aabb.width()) / 2) as i32
            }
            _ => 0,
        },
        match v_align {
            VerticalAlign::Bottom => {
                position.0.y() + (prev_aabb.height() - next_aabb.height()) as i32
            }
            VerticalAlign::Vcentre => {
                position.0.y() + (prev_aabb.height() - next_aabb.height() / 2) as i32
            }
            _ => 0,
        },
    );
}
