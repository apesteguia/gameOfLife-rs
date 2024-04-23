use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub state: State,
}

impl Default for Cell {
    fn default() -> Self {
        Self { state: State::Dead }
    }
}

pub struct World {
    pub cells: Vec<Vec<Cell>>,
    pub size: usize,
    pub alive: i32,
    pub generation: i32,
}

impl World {
    pub fn new(size: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(size);
        for _ in 0..size {
            let row: Vec<Cell> = vec![Cell::default(); size];
            cells.push(row);
        }

        Self {
            cells,
            size,
            alive: 0,
            generation: 0,
        }
    }

    pub fn randomize(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..80) % 7 == 0 {
                    self.cells[i][j].state = State::Alive;
                }
            }
        }
    }

    fn check(&self, x: usize, y: usize) -> State {
        let mut counter = 0;
        let n_w = self.size;
        let n_h = self.size;

        for i in (x as isize - 1)..=(x as isize + 1) {
            for j in (y as isize - 1)..=(y as isize + 1) {
                if i >= 0 && i < n_w as isize && j >= 0 && j < n_h as isize {
                    let i = i as usize;
                    let j = j as usize;
                    if (i != x || j != y) && self.cells[i][j].state == State::Alive {
                        counter += 1;
                    }
                } else {
                    let wrap_i = ((i + n_w as isize) % n_w as isize) as usize;
                    let wrap_j = ((j + n_h as isize) % n_h as isize) as usize;
                    if self.cells[wrap_i][wrap_j].state == State::Alive {
                        counter += 1;
                    }
                }
            }
        }

        if counter == 3 || (counter == 2 && self.cells[x][y].state == State::Alive) {
            State::Alive
        } else {
            State::Dead
        }
    }

    pub fn update(&mut self) {
        let n_w = self.size;
        let n_h = self.size;

        self.generation += 1;
        self.alive = 0;

        for i in 1..(n_w - 1) {
            for j in 1..(n_h - 1) {
                let s = self.check(i, j);
                self.cells[i][j].state = s;
                if s == State::Alive {
                    self.alive += 1;
                }
            }
        }
    }
}
