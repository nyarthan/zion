import { expect, it } from 'vitest';

import { add, multiply, subtract } from './math';

it('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it('subtracts', () => {
  expect(subtract(3, 2)).toBe(1);
});

it('multiplies', () => {
  expect(multiply(3, 3)).toBe(9);
});
