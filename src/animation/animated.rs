use crate::{
    components::{Position, Sprite},
    resources::SpriteSheet,
};
use specs::Entity;

pub struct Animated<'a> {
    pub entity: Entity,
    pub position: &'a mut Position,
    pub sprite: &'a mut Sprite,
    pub sprite_sheet: &'a SpriteSheet,
}

impl<'a> Animated<'a> {
    pub fn new(
        entity: Entity,
        position: &'a mut Position,
        sprite: &'a mut Sprite,
        sprite_sheet: &'a SpriteSheet,
    ) -> Self {
        Animated {
            entity,
            position,
            sprite,
            sprite_sheet,
        }
    }
}
