use crate::action::Action;

use rand::rng;
use rand::seq::IndexedRandom;

#[derive(Debug)]
pub struct Agent {
    pub brain: [[f32; 4]; 240]
}

impl Agent {
    pub fn new() -> Self {
        Agent { brain: [[0.0; 4]; 240]}
    }
    pub fn decide(&self) -> Action {
        let mut rng = rng();
        let actions: [Action; 4] = [Action::Left, Action::Up, Action::Right, Action::Down];
        let choose = actions.choose(&mut rng).cloned().unwrap();

        println!("{:?}", choose);
        choose
    }
}