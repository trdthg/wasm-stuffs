import * as wasm from 'wasm-game-of-life';
import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg.wasm'
const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

// const pre = document.getElementById("canvas");
// const universe = wasm.Universe.new(64, 64);
// const renderLoop = () => {
//     pre.textContent = universe.render();
//     universe.tick()
//     requestAnimationFrame(renderLoop)
// }
// requestAnimationFrame(renderLoop)

const universe = wasm.Universe.new(64, 64);
const width = universe.width();
const height = universe.height();

// 边框也占据一个像素
const canvas = document.getElementById('canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

var fps = 30;
var interval = 1000/fps;
const renderLoop = () => {
  setTimeout(function() {
    universe.tick();
    // drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
  }, interval)
};

// 绘制网格
const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }
  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();
};

const getIndex = (row, col) => {
  return row * width + col;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  // const cells = new Uint8Array(memory.buffer, cellsPtr, width*height);
  const cells = new Uint8Array(memory.buffer, cellsPtr, width*height / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row ++) {
    for (let col = 0; col < height; col ++) {
      const idx = getIndex(row, col);
      // console.log(row, col, idx, bitIsSet(idx, cells));
      // ctx.fillStyle = cells[idx] === wasm.Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillStyle = bitIsSet(idx, cells) === true ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
          row * (CELL_SIZE + 1)+1,
          col * (CELL_SIZE + 1)+1,
          CELL_SIZE,
          CELL_SIZE);
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
