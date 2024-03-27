export function add(a: number, b: number) {
  return a + b;
}

export function subtract(a: number, b: number) {
  return a - b;
}

export function multiply(a: number, b: number) {
  return (Array.from({ length: a }) as number[]).reduce(add.bind({}, b), 0);
}
