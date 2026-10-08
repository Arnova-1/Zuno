use crate::actions::Action;
use crate::maze::Maze;

#[derive(Default, Debug, Copy, Clone)]
pub struct State {
    pub pos: (i32, i32),
    pub charge: i32,
    pub is_shutdown: bool,
    pub is_battery: bool
}

impl State {
    pub fn reset(&mut self) {
        self.pos = (0, 0);
        self.charge = 12;
        self.is_shutdown = false;
        self.is_battery = false;
    }
    pub fn step(&mut self, action: &Action, maze: &Maze, prev_state: &State) -> f32 {
        let (dx, dy) = match action {
            Action::Up => (-1, 0),
            Action::Right => (0, 1),
            Action::Down => (1, 0),
            Action::Left => (0, -1)
        };

        let next_step = (self.pos.0 + dx, self.pos.1 + dy); // (row + dx, column + dy)

        let is_walls = if maze.is_valid_move(next_step) {
            self.pos = next_step;
            false
        } else {
            self.pos = prev_state.pos;
            true
        };

        self.charge = (self.charge -1).max(0);

        self.is_shutdown = self.charge == 0;
        self.is_battery = maze.has_battery(self.pos);

        self.grade(is_walls)
    }
    fn grade(&mut self, is_walls: bool) -> f32 {
        if self.is_battery {
            self.is_shutdown = false;
            10.0
        } else if self.is_shutdown {
            -5.0
        } else if is_walls {
            -3.0
        } else {
            -1.0
        }
    }
}