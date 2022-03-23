use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let size = width * height;
        let mut cells = FixedBitSet::with_capacity(size as usize);
        for i in 0..size {
            cells.set(
                i as usize,
                if js_sys::Math::random() < 0.5 {
                    true
                } else {
                    false
                },
            )
        }
        Self {
            width,
            height,
            cells,
        }
    }

    fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for cell in cells {
            let idx = self.get_index(cell.0, cell.1);
            self.cells.set(idx, true);
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn count_live_neighbor(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = FixedBitSet::with_capacity(self.width as usize * self.height as usize);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                next.set(
                    idx,
                    match (cell, self.count_live_neighbor(row, col)) {
                        (true, x) if x <= 1 => false,
                        (true, 2 | 3) => true,
                        (true, x) if x >= 4 => false,
                        (false, 3) => true,
                        (cell, _) => cell,
                    },
                );
            }
        }
        self.cells = next
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    pub fn input_spaceship() -> Universe {
        let mut universe = Universe::new(6, 6);
        universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        universe
    }
    #[cfg(test)]
    pub fn expected_spaceship() -> Universe {
        let mut universe = Universe::new(6, 6);
        universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
        universe
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    pub fn test_tick() {
        // Let's create a smaller Universe with a small spaceship to test!
        let mut input_universe = input_spaceship();

        // This is what our spaceship should look like
        // after one tick in our universe.
        let expected_universe = expected_spaceship();

        // Call `tick` and then see if the cells in the `Universe`s are the same.
        input_universe.tick();
        assert_eq!(&input_universe.cells, &expected_universe.cells);
    }
}
