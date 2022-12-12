class ComputingUnit {
  cycleNumber = 1;
  registerX = 1;
  signalSum = 0;
  image = "";

  nextTick() {
    if ((this.cycleNumber + 20) % 40 === 0) {
      // console.log(this.cycleNumber, this.registerX);
      this.signalSum += this.cycleNumber * this.registerX;
    }

    if (
      (this.cycleNumber - 1) % 40 >= this.registerX - 1 &&
      (this.cycleNumber - 1) % 40 <= this.registerX + 1
    ) {
      this.image += "#";
    } else {
      this.image += ".";
    }

    this.cycleNumber += 1;
  }

  noop() {
    this.nextTick();
  }

  addx(v) {
    this.nextTick();
    this.nextTick();
    this.registerX += v;
  }
}

const lines = require("fs").readFileSync("./input.txt", "utf8").split("\n");
const cu = new ComputingUnit();
for (const line of lines) {
  if (line === "noop") cu.noop();
  else {
    cu.addx(Number(line.split(" ")[1]));
  }
}

console.log(cu.signalSum);
for (let i = 0; i < 6; i++) {
  console.log(cu.image.slice(i * 40, (i + 1) * 40));
}
