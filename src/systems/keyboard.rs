use specs::prelude::*;

use crate::trust::KeyEvent;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (ReadExpect<'a, Vec<KeyEvent>>,);

    fn run(&mut self, (inputs,): Self::SystemData) {
        for input in &*inputs {
            println!("{:#?}", input);
        }
    }
}
