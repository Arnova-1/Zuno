#[derive(Default, Debug)]
pub struct Maze {
    pub size: (i32, i32),
    pub walls: Vec<(i32, i32)>,
    pub batteries: (i32, i32)
}

impl Maze {
    pub fn init() -> Self {
        Self {
            size: (4, 5),
            walls: vec![(0, 3), (1, 1), (2, 3), (3, 1)],
            batteries: (3, 4)
        }
    }
    pub fn is_inside(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 &&
            pos.0 < self.size.0 &&
            pos.1 >= 0 &&
            pos.1 < self.size.1
    }
    pub fn is_walls(&self, pos: (i32, i32)) -> bool {
        self.walls.contains(&pos)
    }
    pub fn is_valid_move(&self, pos: (i32, i32)) -> bool {
        self.is_inside(pos) && !self.is_walls(pos)
    }
    pub fn has_battery(&self, pos: (i32, i32)) -> bool {
        self.batteries == pos
    }
}