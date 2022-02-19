mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen] // it gets exposed to JS
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/**
 * Lightweight spaceship
 * https://conwaylife.com/wiki/List_of_common_spaceships
 */
fn create_spaceship(cells: &mut FixedBitSet, width: u32, height: u32, v_offset: u32) -> () {
    for row in 0..height {
        for col in 0..width {
            let idx: usize = (row * width + col) as usize;
            let spaceship_cell = match (row - v_offset, col) {
                (0, 0) | (1, 0) | (2, 0) | (3, 1) | (0, 1) | (0, 2) | (0, 3) | (1, 4) | (3, 4) => {
                    true
                }
                (_, _) => cells[idx],
            };
            cells.set(idx, spaceship_cell);
        }
    }
}

fn get_random_u8() -> u8 {
    let mut rand = [0];
    let res = match getrandom::getrandom(&mut rand) {
        Ok(()) => rand,
        _ => [1],
    };
    res[0]
}

#[wasm_bindgen] // it gets exposed to JS
impl Universe {
    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            let rand: u8 = get_random_u8();
            if rand % 2 == 0 {
                cells.set(i, true)
            } else {
                cells.set(i, false)
            }
        }
        // create_spaceship(&mut cells, width, height, height / 2);
        // create_spaceship(&mut cells, width, height, 4);
        // create_spaceship(&mut cells, width, height, height - 4);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // Match beautiful.
                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // https://rustwasm.github.io/docs/book/game-of-life/implementing.html
        // The array members are distance. Super smart!
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if self.cells[idx] { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
