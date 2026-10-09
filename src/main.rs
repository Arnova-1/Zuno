use std::time::Duration;
use crate::actions::Action;
use crate::agent::Agent;
use crate::maze::Maze;
use crate::render::render;
use crate::state::State;

mod maze;
mod actions;
mod agent;
mod render;
mod state;

const ACTIONS: [Action; 4] = [Action::Up, Action::Right, Action::Down, Action::Left];

fn main() {
    let maze = Maze::init();
    let mut agent = Agent::new();

    let mut state = State::default();

    state.reset();

    let mut steps = 0;
    let mut n = 0;

    print!("\x1B[3J\x1B[2J\x1B[H");

    loop {
        let prev_state = state.clone();

        let next_action = agent.decide(&state);

        let (reward, is_wall) = state.step(&ACTIONS[next_action], &maze, &prev_state);

        agent.learn(reward, next_action, &state, &prev_state);

        steps += 1;


        if state.is_shutdown || state.is_battery {
            n += 1;
            agent.end_episode();
            state.reset();
            std::thread::sleep(Duration::from_millis(200))
        }

        if n > 999 {
            render(&maze, &state, steps, ACTIONS[next_action], n, is_wall);
            std::thread::sleep(Duration::from_millis(500))
        }

        if n == 1005 {
            break
        }
    }
}
