use crate::environment::Maze;

pub fn render(maze: &Maze) {
    let w = 5;
    let h = 4;

    let row: String = (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    let pos = maze.pos.0 * w + maze.pos.1;
                    let grid_pos = y * w + x;
                    match grid_pos {
                        3 | 6 | 13 | 16 => "*", // Inner Wall
                        19 => "B", // Battery
                        _ if pos == grid_pos  => "Z", // Zuno Position
                        _ => "-",
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("*\n*");
    let border = "*".repeat((w + 2) as usize);

    println!("{border}\n*{row}*\n{border}")
}