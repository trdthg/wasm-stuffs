mod cells;
mod utils;

use std::fmt::Display;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // 单元只占据一个byte，使用默认的布局会占用4个byte // 好像现在默认也是一个byte
pub enum Cell {
    Dead = 0, // 这里标出具体的数量数为了方便计算总存活数而已
    Alive = 1,
}

// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: Vec<Cell>,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}
#[wasm_bindgen]
impl Universe {
    /// 创建一个新的宇宙
    ///
    /// 现在初始存活状态是固定的
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
        // let cells = (0..width * height)
        //     .map(|_| {
        // if js_sys::Math::random() < 0.5 {
        //     Cell::Alive
        // } else {
        //     Cell::Dead
        // }
        //         // if i % 2 == 0 || i % 7 == 0 {
        //         //     Cell::Alive
        //         // } else {
        //         //     Cell::Dead
        //         // }
        //     })
        //     .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    /// 把宇宙渲染成字符串
    ///
    /// 通过实现Display方法实现
    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    /// 计算线型存储的实际索引
    ///
    /// 这里使用线性空间存储是为了适配wasm的线型内存空间
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// 计算一个点存活的邻居数量，邻居是周围8个点
    ///
    /// 上边界和下边界互相连通，因此没有使用-1，实现了循环，
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

    /// 下一秒的宇宙
    ///
    /// 主要变换逻辑都在这里了
    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();
        let mut next = FixedBitSet::with_capacity(self.width as usize * self.height as usize);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                // next[idx] = match (cell, self.count_live_neighbor(row, col)) {
                //     (Cell::Alive, x) if x <= 1 => Cell::Dead,
                //     (Cell::Alive, 2 | 3) => Cell::Alive,
                //     (Cell::Alive, x) if x >= 4 => Cell::Dead,
                //     (Cell::Dead, 3) => Cell::Alive,
                //     (cell, _) => cell,
                // };
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
    // pub fn cells(&self) -> *const Cell {
    //     self.cells.as_ptr()
    // }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

// impl Display for Universe {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // for line in self.cells.chunks(self.width as usize) {
//         for line in self.cells.chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod test {
    // use super::*;

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

    #[test]
    fn get_index() {}
}
