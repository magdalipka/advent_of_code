type Item = number | Array<number>;
type Packet = Array<Item>;

function compare(left: Packet, right: Packet): boolean | undefined {
  for (let i = 0; i < Math.max(left.length, right.length); i++) {
    const leftItem = left[i];
    const rightItem = right[i];
    if (leftItem === undefined) return true;

    if (rightItem === undefined) return false;
    if (leftItem === undefined && rightItem === undefined) continue;

    if (typeof leftItem === "number" && typeof rightItem === "number") {
      if (leftItem < rightItem) return true;
      if (leftItem > rightItem) return false;
      if (leftItem === rightItem) {
        console.log("continuing");
        continue;
      }
    }
    const res = compare(
      typeof leftItem === "number" ? [leftItem] : leftItem,
      typeof rightItem === "number" ? [rightItem] : rightItem,
    );

    if (res !== undefined) return res;
  }

  return undefined;
}

const result = require("fs")
  .readFileSync(`./${process.argv[2]}.txt`, "utf-8")
  .split("\n\n")
  .map((p: string) => p.split("\n").map((i) => JSON.parse(i)))
  .map((data: [string, string], index: number) => ({ index: index + 1, data }))
  .filter((p) => compare(p.data[0], p.data[1]))
  .reduce((acc, curr) => acc + curr.index, 0);

console.log(result);
