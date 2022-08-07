use super::{
    animations::Animations, collisions::Collisions, events::Events, nodes::Nodes, scenes::Scenes,
    scrolling::Scrolling,
};
use crate::{
    crust::{action, Action},
    event::EventManager,
    scene::SceneManager,
};
use specs::prelude::*;
use std::sync::mpsc::Receiver;

pub struct ActionExecutor {
    rx: Receiver<Action>,

    scrolling: Scrolling,
}

impl ActionExecutor {
    pub fn new(rx: Receiver<Action>, world: &mut World) -> Self {
        ActionExecutor {
            rx,
            scrolling: Scrolling::new(world),
        }
    }

    pub fn process(
        &self,
        world: &mut World,
        scene_manager: &mut SceneManager,
        event_manager: &mut EventManager,
    ) {
        self.rx
            .try_iter()
            .for_each(|action| self.execute(action, world, scene_manager, event_manager));
    }

    fn execute(
        &self,
        action: Action,
        world: &mut World,
        scene_manager: &mut SceneManager,
        event_manager: &mut EventManager,
    ) {
        match action.action {
            Some(action::Action::Quit(..)) => (),
            Some(action::Action::LoadScene(action)) => Scenes::load(action, scene_manager, world),
            Some(action::Action::CreateSceneNode(action)) => Nodes::create(action, world),
            Some(action::Action::DestroySceneNode(action)) => Nodes::destroy(action, world),
            Some(action::Action::PlayAnimation(action)) => Animations::play(action, world),
            Some(action::Action::StopAnimation(action)) => Animations::stop(action, world),
            Some(action::Action::Scroll(action)) => self.scrolling.scroll(action, world),
            Some(action::Action::OnCollision(action)) => Collisions::on_collision(action, world),
            Some(action::Action::Emit(action)) => Events::emit(action, event_manager),
            _ => (),
        }
    }
}
