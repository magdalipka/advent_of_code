const monkeys = [];

class Monkey {
  inspectCounter = 0;
  constructor(items, expression, test, yes, no, base) {
    this.items = items || [];
    this.inspect = (value) => {
      this.inspectCounter += 1;
      let old = value; // used in eval(expression);

      // return Math.floor(eval(expression) / 3); // part 1
      return eval(expression) % base; // part 2
    };
    this.decide = (value) => {
      return value % test === 0 ? yes : no;
    };
  }

  throw(value) {
    const newValue = this.inspect(value);
    const targetMonkey = this.decide(newValue);
    monkeys[targetMonkey].catch(newValue);
  }

  catch(value) {
    this.items.push(value);
  }

  turn() {
    for (const item of this.items) {
      this.throw(item);
    }
    this.items = [];
  }
}

// init monkeys
const input = require("fs").readFileSync("./input.txt", "utf8");

const tests = input
  .split("\n")
  .filter((l) => l.includes("Test: "))
  .map((l) => Number(l.split(" ").at(-1)));
const base = tests.reduce((acc, item) => acc * item, 1);
console.log({ base });

for (const _monkeyInput of input.split("Monkey")) {
  if (!_monkeyInput.length) continue;
  const monkeyInput = _monkeyInput.split("\n");
  const items = monkeyInput[1]
    .slice("  Starting items: ".length)
    .split(", ")
    .map(Number);
  const expression = monkeyInput[2].slice("  Operation: new = ".length);
  const [test, yes, no] = monkeyInput
    .slice(3)
    .map((l) => Number(l.split(" ").at(-1)));
  const monkey = new Monkey(items, expression, test, yes, no, base);
  monkeys.push(monkey);
}

for (let i = 0; i < 10000; i++) {
  // console.log(i);
  for (const monkey of monkeys) {
    monkey.turn();
  }
}

monkeys.sort((a, b) => b.inspectCounter - a.inspectCounter);
// for (const monkey of monkeys) {
//   console.log(monkey.items);
//   console.log(monkey.inspectCounter);
// }
console.log(monkeys[0].inspectCounter * monkeys[1].inspectCounter);
