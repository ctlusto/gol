extern crate rand;

use rand::Rng;
use std::iter::repeat;
use std::{cmp, thread, time};

const W: usize = 25;
const H: usize = 50;
const DENSITY: f64 = 0.5;
const DELAY: u64 = 150;
const ALIVE_CHAR: char = 'O';
const DEAD_CHAR: char = '\u{00b7}';

fn main() {
    let mut g = Grid::new(W, H).randomize();
    loop {
        g = g.tick();
        g.render();
        sleep();
        clear_term();
    }
}

fn get_random_cell_status(probability: f64) -> bool {
    rand::thread_rng().gen_bool(probability)
}

fn clear_term() {
    println!("{}[2J", 27 as char);
}

fn sleep() {
    let duration = time::Duration::from_millis(DELAY);
    thread::sleep(duration);
}

type Pos = (usize, usize);

type Cells = Vec<bool>;

#[derive(Debug)]
struct Grid {
    cells: Cells,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        let cells: Cells = repeat(false).take(rows * cols).collect();

        Grid {
            cells,
            rows,
            cols,
        }
    }

    fn update(&self, new_cells: Cells) -> Grid {
        Grid {
            cells: new_cells,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn next_generation(&self) -> Cells {
        let mut new_cells: Cells = Vec::new();
        for idx in 0..self.cells.len() {
            new_cells.push(self.should_live(idx));
        }

        new_cells
    }

    fn is_alive(&self, pos: &Pos) -> bool {
        let idx: usize = self.pos_to_idx(&pos);
        self.cells[idx]
    }

    fn should_live(&self, idx: usize) -> bool {
        let pos: Pos = self.idx_to_pos(&idx);
        let live_neighbors: usize = self.count_live_neighbors(&pos);
        live_neighbors == 3 || (self.is_alive(&pos) && live_neighbors == 2)
    }

    fn count_live_neighbors(&self, pos: &Pos) -> usize {
        let min_row: usize;
        if pos.0 == 0 {
            min_row = 0;
        } else{
            min_row = pos.0 - 1;
        }

        let min_col: usize;
        if pos.1 == 0 {
            min_col = 0;
        } else{
            min_col = pos.1 - 1;
        }

        let max_row: usize = cmp::min(pos.0 + 1, self.rows - 1);
        let max_col: usize = cmp::min(pos.1 + 1, self.cols - 1);

        let mut count: usize = 0;
        for r in min_row..=max_row {
            for c in min_col..=max_col {
                if r == pos.0 && c == pos.1 {
                    continue;
                }
                if self.is_alive(&(r, c)) {
                    count += 1;
                }
            }
        }
        
        count
    }

    fn idx_to_pos(&self, idx: &usize) -> Pos {
        (idx / self.cols, idx % self.cols)
    }

    fn pos_to_idx(&self, pos: &Pos) -> usize {
        let (r, c) = pos;
        r * self.cols + c
    }

    fn tick(&self) -> Grid {
        self.update(self.next_generation())
    }

    fn randomize(&self) -> Grid {
        let mut cells: Cells = Vec::new();
        for _ in 0..self.cells.len() {
            cells.push(get_random_cell_status(DENSITY));
        }

        self.update(cells)
    }

    fn render(&self) {
        let mut s = String::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.is_alive(&(r, c)) {
                    true => s.push(ALIVE_CHAR),
                    false => s.push(DEAD_CHAR),
                }
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}

