class Node {
  isDirectory;
  size;
  children;
  name;
  parentNode;
  constructor(name, isDirectory, size) {
    this.name = name;
    this.isDirectory = isDirectory;
    this.size = size;
    this.children = [];
  }

  addChild(node) {
    this.children.push(node);
    node.parentNode = this;
  }

  calculateSize() {
    return (this.size = !this.isDirectory
      ? this.size
      : this.children.reduce((acc, child) => acc + child.calculateSize(), 0));
  }

  traverse(root, f) {
    if (!root) return;
    f(root);
    root.children?.forEach((child) => this.traverse(child, f));
  }
}

const lines = require("fs").readFileSync("./input.txt", "utf8").split("\n");

const tree = new Node("/", true);
let root = tree;

function cd(arg) {
  if (arg === "/") root = tree;
  else if (arg === "..") root = root.parentNode;
  else {
    root = root.children.find((c) => c.name === arg);
  }
}

function ls(line) {
  const [data, name] = line.split(" ");
  root.addChild(
    new Node(name, data === "dir", data !== "dir" ? Number(data) : undefined),
  );
}

lines.forEach((line) => {
  if (line.startsWith("$ ls")) {
    //
  } else if (line.startsWith("$ cd")) {
    cd(line.split(" ")[2]);
  } else {
    ls(line);
  }
});

tree.calculateSize();

// part  1
let sum = 0;
tree.traverse(tree, (node) => {
  if (node.isDirectory && node.size < 100_000) sum += node.size;
});

console.log({ sum });

// part 2
const maxSpace = 40_000_000;
let size = tree.size;
tree.traverse(tree, (node) => {
  if (!node.isDirectory) return;
  if (tree.size - node.size < maxSpace && node.size <= size) size = node.size;
});

console.log({ size });
