mod environment;
mod action;
mod agent;
mod render;

use environment::Maze;
use agent::Agent;
use render::render;

fn main() {
    let mut maze = Maze::default();
    let agent = Agent::new();

    maze.reset();

    loop {
        maze.step(agent.decide());

        render(&maze);
        println!("{:?}", maze);

        if maze.is_shutdown {
            break
        }
    }
}
