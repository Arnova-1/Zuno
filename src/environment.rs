use crate::action::Action;

#[derive(Default, Debug)]
pub struct Maze {
    pub pos: (i32, i32), // (Row, Col)
    pub charge: u32,
    pub previous_pos: (i32, i32), // (Row, Col)
    pub is_battery: bool,
    pub is_shutdown: bool,
}

impl Maze {
    pub fn reset(&mut self) {
        self.pos = (0,0);
        self.charge = 12;
    }
    pub fn step(&mut self, action: Action) {
        self.previous_pos = self.pos;

        let (dx,dy) = match action {
            Action::Left => (0, -1),
            Action::Up => (-1, 0),
            Action::Right => (0, 1),
            Action::Down => (1, 0)
        };

        let next_move = (self.pos.0 + dx, self.pos.1 + dy); // 0 = Row, 1 = Col

        if self.evaluate(next_move) {
            println!("Zuno bumped into a wall!");
            self.pos = self.previous_pos
        } else {
            self.pos = next_move
        }

        self.charge -= 1;
        self.is_shutdown = self.charge == 0;
        self.is_battery = self.pos == (3, 4);
    }
    fn evaluate(&self, step: (i32, i32)) -> bool{
        let out_of_bonds = step.0 < 0 || step.0 > 3 || step.1 < 0 || step.1 > 4;
        let inner_wall = step == (0, 3) || step == (1,1) || step == (2,3) || step == (3,1);

        out_of_bonds || inner_wall
    }
}