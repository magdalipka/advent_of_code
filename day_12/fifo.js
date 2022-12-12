class FifoNode {
  constructor(data, next, prev) {
    this.data = data;
    this.next = next;
    this.prev = prev;
  }
}

class Fifo {
  constructor() {
    this.first = null;
    this.last = null;
  }

  isEmpty() {
    return !Boolean(this.first);
  }

  add(d) {
    const n = new FifoNode(d);
    if (!this.last) {
      this.first = n;
      this.last = n;
    } else {
      this.last.next = n;
      n.prev = this.last;
    }
  }

  pop() {
    const n = this.first;
    if (this.first === this.last) {
      this.first = undefined;
      this.last = undefined;
    } else {
      // TODO
      this.first = this.first.next;
      this.first.prev = undefined;
    }
    return n.data;
  }
}

class Stack {
  constructor() {
    this.items = [];
  }
  add(d) {
    this.items.push(d);
  }
  pop() {
    return this.items.pop();
  }
  isEmpty() {
    return !Boolean(this.items.length);
  }
}

module.exports = Stack;
