use crate::{
    action::ActionQueue,
    components::{Id, Position, Sprite},
    resources::SpriteSheet,
};
use specs::Entity;

pub struct Animated<'a> {
    pub entity: Entity,
    pub id: &'a Id,
    pub position: &'a mut Position,
    pub sprite: &'a mut Sprite,
    pub sprite_sheet: &'a SpriteSheet,
    pub queue: Option<&'a ActionQueue>,
}

impl<'a> Animated<'a> {
    pub fn new(
        entity: Entity,
        id: &'a Id,
        position: &'a mut Position,
        sprite: &'a mut Sprite,
        sprite_sheet: &'a SpriteSheet,
        queue: Option<&'a ActionQueue>,
    ) -> Self {
        Animated {
            entity,
            id,
            position,
            sprite,
            sprite_sheet,
            queue,
        }
    }
}
