use super::{
    animations::Animations, collisions::Collisions, events::Events, nodes::Nodes, scenes::Scenes,
};
use crate::{
    core::SceneManager,
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

    pub fn process(
        &self,
        world: &mut World,
        scene_manager: &mut SceneManager,
        event_manager: &mut EventManager,
    ) {
        self.rx
            .try_iter()
            .for_each(|action| Self::execute(action, world, scene_manager, event_manager));
    }

    fn execute(
        action: Action,
        world: &mut World,
        scene_manager: &mut SceneManager,
        event_manager: &mut EventManager,
    ) {
        match action.action {
            Some(action::Action::Quit(..)) => (),
            Some(action::Action::LoadScene(action)) => Scenes::load(action, scene_manager),
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
