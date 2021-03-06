use std::fmt::Display;

use wasm_bindgen::prelude::*;

use crate::profile::Timer;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // 单元只占据一个byte，使用默认的布局会占用4个byte // 好像现在默认也是一个byte
pub enum Cell {
    Dead = 0, // 这里标出具体的数量数为了方便计算总存活数而已
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Dead => Self::Alive,
            Self::Alive => Self::Dead,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
    dirty_num: u32,
    dirty_cells: Vec<u32>,
}
#[wasm_bindgen]
impl Universe {
    /// 创建一个新的宇宙
    ///
    /// 现在初始存活状态是固定的
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let cells: Vec<Cell> = (0..size)
            .map(|i| {
                // if js_sys::Math::random() < 0.5 {
                //     Cell::Alive
                // } else {
                //     Cell::Dead
                // }
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Self {
            width,
            height,
            cells: cells.clone(),
            next_cells: cells,
            dirty_num: 0,
            dirty_cells: Vec::new(),
        }
    }

    /// 设置某一Cell
    pub fn set_cell(&mut self, row: u32, col: u32, state: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx] = state;
    }

    /// 设置若干Cell状态
    fn set_cells(&mut self, state: Cell, cells: &[(u32, u32)]) {
        for cell in cells {
            let idx = self.get_index(cell.0, cell.1);
            self.cells[idx] = state;
        }
    }

    /// 切换一个Cell的状态
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    /// 把宇宙渲染成字符串
    ///
    /// 通过实现Display方法实现
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// 计算线型存储的实际索引
    ///
    /// 这里使用线性空间存储是为了适配wasm的线型内存空间
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// 计算一个点存活的邻居数量，邻居是周围8个点
    ///
    /// 上边界和下边界互相连通，因此没有使用-1，实现了循环，
    fn count_alive_neighbor(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        // for delta_row in [self.height - 1, 0, 1] {
        //     for delta_col in [self.width - 1, 0, 1] {
        //         if delta_row == 0 && delta_col == 0 {
        //             continue;
        //         }

        //         let neighbor_row = (row + delta_row) % self.height;
        //         let neighbor_col = (col + delta_col) % self.width;
        //         let idx = self.get_index(neighbor_row, neighbor_col);
        //         count += self.cells[idx] as u8;
        //     }
        // }
        // count
        let above = if row == 0 { self.height - 1 } else { row - 1 };
        let below = if row == self.height - 1 { 0 } else { row + 1 };
        let left = if col == 0 { self.width - 1 } else { col - 1 };
        let right = if col == self.width - 1 { 0 } else { col + 1 };
        count += self.cells[self.get_index(above, left)] as u8;
        count += self.cells[self.get_index(above, col)] as u8;
        count += self.cells[self.get_index(above, right)] as u8;
        count += self.cells[self.get_index(row, left)] as u8;
        // count += self.cells[self.get_index(row, col)] as u8;
        count += self.cells[self.get_index(row, right)] as u8;
        count += self.cells[self.get_index(below, left)] as u8;
        count += self.cells[self.get_index(below, col)] as u8;
        count += self.cells[self.get_index(below, right)] as u8;
        count
    }

    /// 下一秒的宇宙
    ///
    /// 主要变换逻辑都在这里了
    pub fn tick(&mut self) {
        // 使用drop实现计时
        let _timer = Timer::new("Universe::tick");
        // let mut next = {
        //     // let _timer = Timer::new("step1: allocate new cells");
        //     self.cells.clone()
        // };
        // let mut dirtys = Vec::new();
        self.dirty_num = 0;
        self.dirty_cells.clear();
        {
            let _timer = Timer::new("step2: calculate cells");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    self.next_cells[idx] = match (cell, self.count_alive_neighbor(row, col)) {
                        (Cell::Alive, x) if x <= 1 => {
                            self.dirty_cells.push(row);
                            self.dirty_cells.push(col);
                            self.dirty_num += 1;
                            Cell::Dead
                        }
                        (Cell::Alive, 2 | 3) => Cell::Alive,
                        (Cell::Alive, x) if x >= 4 => {
                            self.dirty_cells.push(row);
                            self.dirty_cells.push(col);
                            self.dirty_num += 1;
                            Cell::Dead
                        }
                        (Cell::Dead, 3) => {
                            self.dirty_cells.push(row);
                            self.dirty_cells.push(col);
                            self.dirty_num += 1;
                            Cell::Alive
                        }
                        (cell, _) => cell,
                    };
                }
            }
        }
        // self.cells = next.clone();
        // std::mem::swap(&mut self.cells, &mut self.next_cells);
        let _timer = Timer::new("step2: exchange cells and next_cells");
        std::mem::swap(&mut self.cells, &mut self.next_cells);
        // self.cells = self.next_cells.clone()
    }
}

/// 优化
///
/// 把cells的指针和长宽暴露给js, js直接读取wasm内存进行渲染
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn dirty_cells(&self) -> *const u32 {
        self.dirty_cells.as_ptr()
    }
    pub fn dirty_num(&self) -> u32 {
        self.dirty_num
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::mem::size_of;

    #[test]
    fn cell_size() {
        #[repr(u32)] // 改变默认的内存布局
        #[derive(Debug, Clone, Copy)]
        pub enum Cell {
            Dead = 1, // 这里标出具体的数量数为了方便计算总存活数而已
            Alive = 0,
        }
        let cell_alive = Cell::Alive;
        let cell_dead = Cell::Dead;
        println!("type: {:?} int: {:?}", cell_alive, cell_alive as u8,);
        println!("type: {:?} int: {:?}", cell_dead, cell_dead as u8,);
        assert_eq!(size_of::<Cell>(), 4);
    }
}
