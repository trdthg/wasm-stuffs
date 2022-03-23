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

let animationId = null;
const playPauseButton = document.getElementById("play-pause");

playPauseButton.addEventListener("click", () => {
  if (animationId === null) {
    playPauseButton.textContent = "暂停";
    renderLoop();
  } else {
    playPauseButton.textContent = "继续";
    cancelAnimationFrame(animationId);
    animationId = null;
  }
});
canvas.addEventListener("click", e => {
  let boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width
  const scaleY = canvas.height / boundingRect.height

  const canvasLeft = (e.clientX - boundingRect.left) * scaleX
  const canvasTop = (e.clientY - boundingRect.top) * scaleY
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1)
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1)
  if (e.ctrlKey === true) {
    universe.set_cell(row-1, col, wasm.Cell.Alive);
    universe.set_cell(row, col-1, wasm.Cell.Alive);
    universe.set_cell(row+1, col-1, wasm.Cell.Alive);
    universe.set_cell(row+1, col, wasm.Cell.Alive);
    universe.set_cell(row+1, col+1, wasm.Cell.Alive);
  } else {
    universe.toggle_cell(row, col);
  }
  drawGrid();
  drawCells();
})

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const renderLoop = () => {
  fps.render(); //new

  universe.tick();
  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

// document.getElementById("fpsinput").addEventListener('change', e => {
//   fps = e.target.value;
//   interval = 1000/fps;
//   document.getElementById("fps").textContent = "fps: " + fps
// });

// var fps = 60;
// var interval = 1000/fps;
// let last = new Date().getTime();
// const renderLoop = () => {
//   animationId = requestAnimationFrame(renderLoop);
//   let now = new Date().getTime();
//   let elapsed = now - last;
//   if (elapsed > interval) {
//     last = now - (elapsed % interval);
//     universe.tick();
//     drawCells();
//   }
// };

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

// const bitIsSet = (n, arr) => {
//   const byte = Math.floor(n / 8);
//   const mask = 1 << (n % 8);
//   return (arr[byte] & mask) === mask;
// };

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width*height);
  // const cells = new Uint8Array(memory.buffer, cellsPtr, width*height / 8);
  ctx.beginPath();

  // 优化性能，ctx.fillStyle赋值这句话消耗的时间和fill几乎一样，占据了大量时间
  // 这里优化之后，ctx.fillStyle几乎不再消耗性能
  // Alive cells.
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] !== wasm.Cell.Alive) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }


  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row ++) {
    for (let col = 0; col < height; col ++) {
      const idx = getIndex(row, col);
      // console.log(row, col, idx, bitIsSet(idx, cells));
      // ctx.fillStyle = cells[idx] === wasm.Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
      // ctx.fillStyle = bitIsSet(idx, cells) === true ? ALIVE_COLOR : DEAD_COLOR;
      if (cells[idx] !== wasm.Cell.Dead) {
        continue;
      }
      ctx.fillRect(
          col * (CELL_SIZE + 1)+1,
          row * (CELL_SIZE + 1)+1,
          CELL_SIZE,
          CELL_SIZE);
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
