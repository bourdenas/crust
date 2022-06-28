use super::{animations::Animations, collisions::Collisions, events::Events, nodes::Nodes};
use crate::{
    crust::{action, Action},
    event::EventManager,
};
use specs::prelude::*;
use std::sync::mpsc::Receiver;

pub struct ActionExecutor {
    rx: Receiver<Action>,
}

impl ActionExecutor {
    pub fn new(rx: Receiver<Action>) -> Self {
        ActionExecutor { rx }
    }

    pub fn process(&self, world: &mut World, event_manager: &mut EventManager) {
        self.rx
            .try_iter()
            .for_each(|action| Self::execute(action, world, event_manager));
    }

    fn execute(action: Action, world: &mut World, event_manager: &mut EventManager) {
        match action.action {
            Some(action::Action::Quit(..)) => (),
            Some(action::Action::CreateSceneNode(action)) => Nodes::create(action, world),
            Some(action::Action::DestroySceneNode(action)) => Nodes::destroy(action, world),
            Some(action::Action::PlayAnimation(action)) => Animations::play(action, world),
            Some(action::Action::StopAnimation(action)) => Animations::stop(action, world),
            Some(action::Action::OnCollision(action)) => Collisions::on_collision(action, world),
            Some(action::Action::Emit(action)) => Events::emit(action, event_manager),
            _ => (),
        }
    }
}
