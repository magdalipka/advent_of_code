const trees = require("fs")
  .readFileSync("./input.txt", "utf8")
  .split("\n")
  .map((l) => l.split("").map((tree) => Number(tree)));

const rows = trees.length - 1;
const columns = trees[0].length;

let counter = 0;
let maxScore = 0;
trees.forEach((line, rowNumber) => {
  line.forEach((treeHeight, columnNumber) => {
    if (isVisible(rowNumber, columnNumber, treeHeight)) {
      counter += 1;
    }
    let score = scenicScore(rowNumber, columnNumber, treeHeight);
    if (score > maxScore) {
      maxScore = score;
    }
  });
});

function scenicScore(rowNumber, columnNumber, height) {
  if (
    rowNumber == 0 ||
    rowNumber === rows - 1 ||
    columnNumber === 0 ||
    columnNumber === columns - 1
  )
    return 0;

  let leftScore = 0;
  let leftIndex = columnNumber;
  do {
    leftIndex -= 1;
    leftScore += 1;
  } while (leftIndex > 0 && trees[rowNumber][leftIndex] < height);

  let rightScore = 0;
  let rightIndex = columnNumber;
  do {
    rightIndex += 1;
    rightScore += 1;
  } while (rightIndex < columns - 1 && trees[rowNumber][rightIndex] < height);

  let topScore = 0;
  let topIndex = rowNumber;
  do {
    topIndex -= 1;
    topScore += 1;
  } while (topIndex > 0 && trees[topIndex][columnNumber] < height);
  let bottomScore = 0;
  let bottomIndex = rowNumber;

  do {
    bottomIndex += 1;
    bottomScore += 1;
  } while (
    bottomIndex < columns - 1 &&
    trees[bottomIndex][columnNumber] < height
  );

  return leftScore * rightScore * topScore * bottomScore;
}

function isVisible(rowNumber, columnNumber, height) {
  let visibleLeft = true;
  for (let i = 0; i < columnNumber; i++) {
    if (trees[rowNumber][i] >= height) {
      visibleLeft = false;
      break;
    }
  }
  let visibleRight = true;
  if (columnNumber !== columns - 1) {
    for (let i = columnNumber + 1; i < columns; i++) {
      if (trees[rowNumber][i] >= height) {
        visibleRight = false;
        break;
      }
    }
  }
  let visibleTop = true;
  for (let i = 0; i < rowNumber; i++) {
    if (trees[i][columnNumber] >= height) {
      visibleTop = false;
      break;
    }
  }
  let visibleBottom = true;
  if (rowNumber !== rows - 1) {
    for (let i = rowNumber + 1; i < rows; i++) {
      if (trees[i][columnNumber] >= height) {
        visibleBottom = false;
        break;
      }
    }
  }

  return visibleLeft || visibleRight || visibleTop || visibleBottom;
}

console.log({ counter, maxScore });
