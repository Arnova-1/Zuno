use crate::actions::Action;
use crate::maze::Maze;
use crate::state::State;

pub fn render(maze: &Maze, state: &State, steps: i32, next_action: Action, episode: i32, is_wall: bool) {
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

    let bump = if is_wall { "Zuno bumped into a wall!" } else { "" };

    println!("Zuno turned {:?} on the next step\nSteps {steps} Episode {episode}: \n┏{border}┓\n┃{row}┃\n┗{border}┛\n{bump}", next_action);
    print!("\x1B[3J\x1B[2J\x1B[H");
}