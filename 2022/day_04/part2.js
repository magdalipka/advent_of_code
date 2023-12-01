const fs = require("fs");

const content = fs.readFileSync("./input.txt", "utf8");

function contains(a, b) {
  return (a[0] <= b && b <= a[1]) || a[0] >= b >= a[1];
}

const res = content
  .split("\n")
  .map((line) => line.split(",").map((e) => e.split("-").map(Number)))
  .map(
    ([a, b]) =>
      a &&
      b &&
      (contains(a, b[0]) ||
        contains(a, b[1]) ||
        contains(b, a[0]) ||
        contains(b, a[1]))
  )
  .filter(Boolean);

console.log(res.length);
