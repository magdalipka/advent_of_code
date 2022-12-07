const data = require("fs").readFileSync("./input.txt", "utf8");

function detect(length) {
  for (let i = 0; i < data.length - length; i++) {
    if (Array.from(new Set(data.slice(i, i + length))).length === length) {
      console.log(i + length);
      break;
    }
  }
}

detect(4); // part 1
detect(14); // part 2
