const fs = require("fs");

class Warehouse {
  stacks;

  _sanitizeInput = (input) =>
    input.map((line) =>
      Array(Math.ceil(line.length / 4))
        .fill()
        .map((_, index) => index * 4)
        .map((begin) => line.slice(begin, begin + 4))
        .map((i) => (i[1].match(/[A-Z]/i) ? i[1] : undefined)),
    );

  constructor(numberOfStacks, initialStackHeight, _input) {
    this.stacks = new Array(numberOfStacks).fill(1).map(() => []);
    const input = this._sanitizeInput(_input);
    for (let i = 0; i < numberOfStacks; i++) {
      for (let j = 0; j < initialStackHeight; j++) {
        this.stacks[i][j] = input[initialStackHeight - j - 1][i];
      }
      this.stacks[i] = this.stacks[i].filter(Boolean);
    }
  }

  moveForPart1(source, target, iter) {
    if (!iter) return;
    const elem = this.stacks[source - 1].pop();
    this.stacks[target - 1].push(elem);
    this.moveForPart1(source, target, iter - 1);
  }

  moveForPart2(source, target, iter) {
    if (!iter) return;
    const elem = this.stacks[source - 1].pop();
    this.moveForPart2(source, target, iter - 1);
    this.stacks[target - 1].push(elem);
  }

  printTops() {
    console.log(this.stacks.map((stack) => stack.at(-1)).join(""));
  }
}

const lines = fs.readFileSync("./input.txt", "utf8").split("\n");

const initialStackHeight = lines.filter((line) =>
  line.split("").includes("["),
).length;

const numberOfStacks = lines[initialStackHeight]
  .split("")
  .reduce((acc, curr) => (Number(curr) ? Number(curr) : acc), 0);

const warehouseForPart1 = new Warehouse(
  numberOfStacks,
  initialStackHeight,
  lines.slice(0, initialStackHeight),
);
const warehouseForPart2 = new Warehouse(
  numberOfStacks,
  initialStackHeight,
  lines.slice(0, initialStackHeight),
);

lines.splice(0, initialStackHeight + 2);
lines.pop();

lines
  .map((line) => line.split(" "))
  .map((line) =>
    warehouseForPart1.moveForPart1(
      Number(line[3]),
      Number(line[5]),
      Number(line[1]),
    ),
  );

lines
  .map((line) => line.split(" "))
  .map((line) =>
    warehouseForPart2.moveForPart2(
      Number(line[3]),
      Number(line[5]),
      Number(line[1]),
    ),
  );

warehouseForPart1.printTops();
warehouseForPart2.printTops();
