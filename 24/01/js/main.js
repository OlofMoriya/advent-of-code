const file = Bun.file("../../01/01");

const content = await file.text();
const lines = content.split("\n");

const left = [];
const right = [];
lines.forEach((line) => {
  if (line) {
    const [l, r] = line.split(/\s+/);
    left.push(l);
    right.push(r);
  }
});

const products = left.map((p) => {
  return p * right.filter((r) => r === p).length;
});

const sum = products.reduce((a, b) => a + b, 0);
console.log(sum);
