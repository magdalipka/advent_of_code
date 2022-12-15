const input = `498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9`
  .split("\n")
  .map((row: string) =>
    row
      .split(" -> ")
      .map((coordinates: string) => coordinates.split(",").map(Number)),
  );

const board = new Array(200).fill(1).map(() => new Array(700).fill(" "));
let floorLevel = 0;

for (const row of input) {
  // console.log("                ROW");
  let [currentY, currentX] = row[0];
  board[currentX][currentY] = "r";
  // console.log({ currentX, currentY });

  for (const [destinationY, destinationX] of row) {
    // console.log("                COORD");
    while (currentX !== destinationX || currentY !== destinationY) {
      if (currentX < destinationX) currentX += 1;
      else if (currentX > destinationX) currentX -= 1;
      if (currentY < destinationY) currentY += 1;
      else if (currentY > destinationY) currentY -= 1;
      // console.log({ currentX, currentY });
      board[currentX][currentY] = "r";
      if (currentX > floorLevel) floorLevel = currentX;
    }
  }
}

floorLevel += 2;

for (let i = 0; i < board[0].length; i++) {
  board[floorLevel][i] = "r";
}

class SandParticle {
  index: number;
  x: number;
  y: number;
  constructor(index: number) {
    this.index = index;
    this.x = 0;
    this.y = 500;
  }
  move(): boolean | null {
    if (this.x >= board.length - 1) {
      return null;
    } else if (board[this.x + 1][this.y] === " ") {
      this.x = this.x + 1;
      return true;
    } else if (board[this.x + 1][this.y - 1] === " ") {
      this.x = this.x + 1;
      this.y = this.y - 1;
      return true;
    } else if (board[this.x + 1][this.y + 1] === " ") {
      this.x = this.x + 1;
      this.y = this.y + 1;
      return true;
    } else {
      return false;
    }
  }
}

const sandGenerator = {
  sourceCoord: { x: 0, y: 500 },
  lastIndex: 0,
  activeSandParticles: [] as Array<SandParticle>,
};

const canvas = document.getElementById("Main") as HTMLCanvasElement;
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
const ctx = canvas.getContext("2d");

function animate() {
  sandGenerator.activeSandParticles.push(
    new SandParticle(sandGenerator.lastIndex++),
  );

  canvas.width = 1000;
  canvas.height = 1000;
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  for (let rowIndex = 0; rowIndex < board.length; rowIndex++) {
    for (let columnIndex = 0; columnIndex < board[0].length; columnIndex++) {
      const cell = board[rowIndex][columnIndex];
      if (cell === "r") {
        ctx.fillStyle = "black";
        ctx.fillRect(columnIndex, rowIndex, 1, 1);
      }
      if (cell === "s") {
        ctx.fillStyle = "#00ff00";
        ctx.fillRect(columnIndex, rowIndex, 1, 1);
      }
    }
  }

  ctx.fillStyle = "#0000ff";
  ctx.fillRect(sandGenerator.sourceCoord.y, sandGenerator.sourceCoord.x, 1, 1);

  ctx.fillStyle = "#ff0000";
  for (const p of sandGenerator.activeSandParticles) {
    ctx.fillRect(p.y, p.x, 1, 1);
  }

  let next = true;
  console.log(sandGenerator.activeSandParticles.length);
  for (const [index, p] of Object.entries(sandGenerator.activeSandParticles)) {
    // console.log({ index });
    let moved = p.move();
    if (moved === false) {
      let { x, y } = p;
      board[x][y] = "s";
      sandGenerator.activeSandParticles.shift();
    } else if (moved === null) {
      next = false;
      console.log({ index: p.index });
    }
    if (!sandGenerator.activeSandParticles.length) {
      console.log({ index: p.index });
      next = false;
    }
  }

  if (next) {
    // console.log("animating");
    requestAnimationFrame(animate);
  }
}

animate();

// 1329-2112
