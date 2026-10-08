use crate::actions::Action;
use crate::state::State;

const fn q_row(row: i32, column: i32, charge: i32) -> usize {
    (((row * 5) + column) * 13 + charge) as usize
}

const ACTIONS: [Action; 4] = [Action::Up, Action::Right, Action::Down, Action::Left];

pub struct Agent {
    pub brain: [[f32; ACTIONS.len()]; 260],
    pub epsilon: f32,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            brain: [[0.0; ACTIONS.len()]; 260],
            epsilon: 1.0
        }}
    pub fn best(&self, state: &State) -> (f32, usize) {
        let q_row = q_row(state.pos.0, state.pos.1, state.charge);
        
        let weight: Vec<_> = self.brain[q_row].into_iter().zip(0..ACTIONS.len()).collect();

        let best = weight.into_iter().fold(
            (f32::NEG_INFINITY, 0), | best, current | {
                if current.0 > best.0 {
                    current
                } else {
                    best
                }
            }
        );

        best
    }
    pub fn decide(&self, state: &State) -> usize {
        let random_action_index = rand::random_range(0..4);
        let random_number = rand::random_range(0.0..1.0);

        if random_number > self.epsilon {
            self.best(&state).1
        } else {
            random_action_index
        }
    }
    pub fn learn(&mut self, reward: f32, next_action: usize, current_state: &State, prev_state: &State) {
        let learning_rate = 0.1;
        let discount = 0.8;
        let q_row = q_row(prev_state.pos.0, prev_state.pos.1, prev_state.charge);

        let (best, _) = self.best(current_state);

        self.brain[q_row][next_action] = self.brain[q_row][next_action] + learning_rate
            * (reward + discount * best - self.brain[q_row][next_action])
    }
    pub fn end_episode(&mut self) {
        self.epsilon = 0.05_f32.max(self.epsilon * 0.995);
    }
}