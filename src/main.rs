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

    loop {
        let prev_state = state.clone();

        let next_action = agent.decide(&state);

        let reward = state.step(&ACTIONS[next_action], &maze, &prev_state);

        agent.learn(reward, next_action, &state, &prev_state);

        steps += 1;

        println!("=== {} ===\n {:?}", (prev_state.pos.0 * 5 + prev_state.pos.1) * 13 + prev_state.charge, agent.brain[((prev_state.pos.0 * 5 + prev_state.pos.1) * 13 + prev_state.charge) as usize]);
        println!("=== {} ===\n {:?}", (state.pos.0 * 5 + state.pos.1) * 13 + state.charge, agent.brain[((state.pos.0 * 5 + state.pos.1) * 13 + state.charge) as usize]);
        render(&maze, &state, steps, ACTIONS[next_action]);

        if state.is_shutdown || state.is_battery {
            n += 1;
            if state.is_shutdown { println!("Zuno is shutting down..."); } else { println!("Zuno has reached the battery!") }
            agent.end_episode();
            state.reset();
        }

        if n == 100 {
            println!("{}", agent.epsilon);
            break
        }
    }
}
