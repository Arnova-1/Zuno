use crate::actions::Action;
use crate::maze::Maze;
use crate::state::State;

pub fn render(maze: &Maze, state: &State, steps: i32, next_action: Action) {
    let (h, w) = maze.size;

    let row: String = (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    let pos = state.pos.0 * w + state.pos.1;
                    let grid_pos = y * w + x;
                    match grid_pos {
                        3 | 6 | 13 | 16 => " █ ", // Inner Wall
                        19 => " ≡ ", // Battery
                        _ if pos == grid_pos  => " ◉ ", // Zuno Position
                        _ => " ░ ",
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("┃\n┃");
    let border = "━".repeat((w * 3) as usize);

    println!("Zuno took {:?} on the next step\nSteps {steps}: \n┏{border}┓\n┃{row}┃\n┗{border}┛", next_action)
}