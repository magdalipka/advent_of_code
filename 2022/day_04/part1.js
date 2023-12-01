const lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('input.txt')
});

function contains(a, b) {
  console.log({a, b})
  return Number(a[0]) <= Number(b[0]) && Number(a[1]) >= Number(b[1]);
}

let counter= 0;

lineReader.on('line', (line) => {
  
  const elves = line.split(",").map(elf => elf.split("-"))

  if(contains(...elves) || contains(...elves.reverse())) counter +=1; 

});

lineReader.on("close", () => {
  console.log({counter})
})