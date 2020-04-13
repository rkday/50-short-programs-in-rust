#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct GoL {
    pub height: u32,
    pub width: u32,
    state: Vec<CellState>,
}

impl GoL {
    pub fn new(height: u32, width: u32) -> Self {
        let mut g = GoL {
            height,
            width,
            state: Vec::new(),
        };
        for _x in 0..height {
            for _y in 0..width {
                g.state.push(if rand::random::<bool>() {
                    CellState::Alive
                } else {
                    CellState::Dead
                });
            }
        }
        g
    }

    fn cell_state(&self, x: i32, y: i32) -> CellState {
        let actual_x = match x {
            -1 => self.width - 1,
            x if x == self.width as i32 => 0,
            x => x as u32,
        };
        let actual_y = match y {
            -1 => self.height - 1,
            y if y == self.height as i32 => 0,
            y => y as u32,
        };

        let location = actual_x as usize + (actual_y * self.width) as usize;
        self.state[location]
    }

    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.cell_state(x, y) == CellState::Alive
    }

    fn alive_neighbours(&self, x: u32, y: u32) -> u32 {
        let mut ret = 0;

        for y_change in -1..=1 {
            for x_change in -1..=1 {
                if !(y_change == 0 && x_change == 0)
                    && self.is_alive(x as i32 + x_change, y as i32 + y_change)
                {
                    ret += 1
                }
            }
        }

        ret
    }

    pub fn next_state(&self) -> GoL {
        let mut new_states = Vec::with_capacity((self.height * self.width) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                new_states.push(
                    match (
                        self.cell_state(x as i32, y as i32),
                        self.alive_neighbours(x, y),
                    ) {
                        (CellState::Alive, 2) => CellState::Alive,
                        (CellState::Alive, 3) => CellState::Alive,
                        (CellState::Dead, 3) => CellState::Alive,
                        (_, _) => CellState::Dead,
                    },
                )
            }
        }
        GoL {
            height: self.height,
            width: self.width,
            state: new_states,
        }
    }
}

impl std::fmt::Display for GoL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                output.push(if self.is_alive(x, y) { 'O' } else { 'X' })
            }
            output.push('\n')
        }
        write!(f, "{}", output)
    }
}


