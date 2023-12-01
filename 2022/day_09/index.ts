type Direction = "R" | "U" | "L" | "D";

class RopeNode {
  name: string;
  position: [number, number]; // [row][column]
  next?: RopeNode;

  constructor(name?: string) {
    this.position = [0, 0];
    this.name = name || "o";
  }

  move(direction: Direction): void {
    switch (direction) {
      case "U": {
        this.position[0] += 1;
        break;
      }
      case "D": {
        this.position[0] -= 1;
        break;
      }
      case "L": {
        this.position[1] -= 1;
        break;
      }
      case "R": {
        this.position[1] += 1;
        break;
      }
    }
    this.next?.follow(this);
  }

  async follow(previousNode: RopeNode): Promise<void> {
    const distance = Math.sqrt(
      Math.pow(this.position[0] - previousNode.position[0], 2) +
        Math.pow(this.position[1] - previousNode.position[1], 2),
    );
    if (distance === 1) {
      // no move required
      // console.log(distance)
    } else if (
      previousNode.position[0] === this.position[0] ||
      previousNode.position[1] === this.position[1]
    ) {
      // simple move
      if (this.position[0] < previousNode.position[0]) {
        this.position[0] += 1;
      } else if (this.position[0] > previousNode.position[0]) {
        this.position[0] -= 1;
      } else if (this.position[1] < previousNode.position[1]) {
        this.position[1] += 1;
      } else if (this.position[1] > previousNode.position[1]) {
        this.position[1] -= 1;
      }
    } else {
      // diagonal move
      let estimatedMovement = [
        (this.position[0] + previousNode.position[0]) / 2 - this.position[0],
        (this.position[1] + previousNode.position[1]) / 2 - this.position[1],
      ] as const;
      const step = Math.floor(
        Math.max(
          Math.abs(estimatedMovement[0]),
          Math.abs(estimatedMovement[1]),
        ),
      );
      this.position[0] += Math.sign(estimatedMovement[0]) * step;
      this.position[1] += Math.sign(estimatedMovement[1]) * step;
    }

    if (this.next) this.next.follow(this);
  }
}

class Rope {
  head: RopeNode;
  tail: RopeNode;
  move: typeof RopeNode.prototype.move;

  constructor(ropeLength: number) {
    this.head = new RopeNode("H");
    this.move = this.head.move.bind(this.head);

    let lastNode = this.head;
    while (ropeLength) {
      const newNode = new RopeNode();
      lastNode.next = newNode;
      lastNode = newNode;
      ropeLength -= 1;
    }
    this.tail = lastNode;
    this.tail.name = "T";
  }

  print(height: number, width: number) {
    console.log("-".repeat(width + 2));
    const board = new Array(height)
      .fill(1)
      .map(() => new Array(width).fill("."));
    let currentNode: RopeNode | undefined = this.head;
    console.log(currentNode.position);
    while (currentNode) {
      board[currentNode.position[0] + 50][currentNode.position[1] + 50] =
        currentNode.name;
      currentNode = currentNode.next;
    }
    board[this.tail.position[0] + 50][this.tail.position[1] + 50] =
      this.tail.name;
    board[this.head.position[0] + 50][this.head.position[1] + 50] =
      this.head.name;
    for (let row = height - 1; row >= 0; row--) {
      console.log(board[row].join(""));
    }
  }
}

const moves = require("fs")
  .readFileSync("./input.txt", "utf8")
  .split("\n")
  .map((l: string) => l.split(" "));

const rope = new Rope(9);
// rope.print(100, 100);

const tailVisited = new Set<string>();
tailVisited.add("0,0");

const sleep = () =>
  new Promise((res) => {
    setTimeout(() => res("ok"), 200);
  });

async function main() {
  for (const [direction, steps] of moves) {
    for (let i = 0; i < Number(steps); i++) {
      // await sleep();
      rope.move(direction);

      // console.clear();
      // rope.print(100, 100);

      tailVisited.add(rope.tail.position[0] + "," + rope.tail.position[1]);
    }
  }
}

void main();

console.log(tailVisited.size);
