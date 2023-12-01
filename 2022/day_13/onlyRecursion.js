function isRightOrder([leftFirst, ...left], [rightFirst, ...right]) {
  if (leftFirst === undefined && rightFirst === undefined) return undefined;
  if (leftFirst === undefined) return true;
  if (rightFirst === undefined) return false;

  if (typeof leftFirst === "number" && typeof rightFirst === "number") {
    if (leftFirst < rightFirst) return true;
    if (rightFirst < leftFirst) return false;
    const result = isRightOrder(left, right);
    if (result !== undefined) return result;
  }

  if (typeof leftFirst === "number" && typeof rightFirst !== "number") {
    const result = isRightOrder([leftFirst], rightFirst);
    if (result !== undefined) return result;
  }
  if (typeof rightFirst === "number" && typeof leftFirst !== "number") {
    const result = isRightOrder(leftFirst, [rightFirst]);
    if (result !== undefined) return result;
  }

  if (typeof leftFirst !== "number" && typeof rightFirst !== "number") {
    const result = isRightOrder(leftFirst, rightFirst);
    if (result !== undefined) return result;
  }
  return isRightOrder(left, right);
}

const resultPart1 = require("fs")
  .readFileSync(`./input.txt`, "utf-8")
  .split("\n\n")
  .map((p) => p.split("\n").map(JSON.parse))
  .map((data, index) => ({ index: index + 1, data }))
  .filter((p) => isRightOrder(p.data[0], p.data[1]))
  .reduce((acc, curr) => acc + curr.index, 0);

console.log(resultPart1);

const resultPart2 = require("fs")
  .readFileSync(`./input.txt`, "utf-8")
  .concat("\n[[2]]\n[[6]]")
  .split("\n")
  .filter((i) => i.length)
  .map(JSON.parse)
  .sort((a, b) => (isRightOrder(a, b) ? -1 : 1))
  .map(JSON.stringify)
  .map((item, index) => ({ index: index + 1, item }))
  .filter((i) => i.item === "[[2]]" || i.item === "[[6]]")
  .reduce((acc, i) => acc * i.index, 1);

console.log(resultPart2);
