const sleep = async () => new Promise((res) => setTimeout(() => res(1), 1000));

class Field {
  constructor(elevation, minDistance, type, x, y) {
    this.elevation = elevation;
    this.minDistance = minDistance;
    this.type = type;
    this.x = x;
    this.y = y;
  }
}
class Grid {
  constructor(input, reverse) {
    this.fields = input.split("\n").map((l, row) =>
      l.split("").map((e, col) => {
        if (e === "S") {
          const n = new Field(1, reverse ? Number.MAX_VALUE : 0, "s", row, col);
          this.start = n;
          return n;
        } else if (e === "E") {
          const n = new Field(
            26,
            reverse ? 0 : Number.MAX_VALUE,
            "e",
            row,
            col,
          );
          this.end = n;
          return n;
        } else {
          return new Field(
            e.charCodeAt() - 96,
            Number.MAX_VALUE,
            "x",
            row,
            col,
          );
        }
      }),
    );
    this.rows = this.fields.length;
    this.columns = this.fields[0].length;
  }
  print(a, b) {
    const lines = this.fields.map((row) =>
      row.map((f) => String.fromCharCode(f.elevation + 96)),
    );
    lines[a.x][a.y] = "A";
    lines[b.x][b.y] = "B";
    for (const line of lines) {
      console.log(line.join(""));
    }
  }
}

const directions = ["up", "right", "down", "left"];
function getNeighborCoord(x, y, side) {
  switch (side) {
    case "up": {
      return { x, y: y - 1 };
    }
    case "down": {
      return { x, y: y + 1 };
    }
    case "left": {
      return { x: x - 1, y };
    }
    case "right": {
      return { x: x + 1, y: y };
    }
  }
}

async function part1() {
  const grid = new Grid(require("fs").readFileSync("./input.txt", "utf-8"));
  const q = new (require("./fifo"))();

  q.add(grid.start);

  while (!q.isEmpty()) {
    const current = q.pop();
    for (const d of directions) {
      const { x, y } = getNeighborCoord(current.x, current.y, d);
      if (x >= 0 && x < grid.rows && y >= 0 && y < grid.columns) {
        const neighbor = grid.fields[x][y];
        if (
          neighbor.elevation <= current.elevation + 1 &&
          neighbor.minDistance > current.minDistance + 1
        ) {
          // console.clear();
          // console.log("---------");
          // grid.print(current, neighbor);
          // await sleep();

          neighbor.minDistance = current.minDistance + 1;
          q.add(neighbor);
        }
      }
    }
  }
  console.log(grid.end);
}

async function part2() {
  const grid = new Grid(
    require("fs").readFileSync("./input.txt", "utf-8"),
    true,
  );
  const q = new (require("./fifo"))();

  q.add(grid.end);

  while (!q.isEmpty()) {
    const current = q.pop();
    for (const d of directions) {
      const { x, y } = getNeighborCoord(current.x, current.y, d);
      if (x >= 0 && x < grid.rows && y >= 0 && y < grid.columns) {
        const neighbor = grid.fields[x][y];
        if (
          neighbor.elevation >= current.elevation - 1 &&
          neighbor.minDistance > current.minDistance + 1
        ) {
          // console.clear();
          // console.log("---------");
          // grid.print(current, neighbor);
          // await sleep();

          neighbor.minDistance = current.minDistance + 1;
          q.add(neighbor);
        }
      }
    }
  }
  let solution = Number.MAX_VALUE;
  for (const row of grid.fields) {
    for (const field of row) {
      if (field.elevation === 1 && field.minDistance < solution) {
        solution = field.minDistance;
      }
    }
  }
  console.log(solution);
}

void part2();
