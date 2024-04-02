import { expect, it } from 'vitest';

import { add } from './math';

it('adds', (ctx) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (ctx = 1 as any) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task }) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task } = 1 as any) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task = 1 as any }) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task: alias }) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ ...ctx }) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task, ...ctx }) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task: alias, ...ctx }) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[ctx]) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[ctx = 1 as any]) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[{ task }]) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[{ task: alias }]) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[{ ...ctx }]) => {
  console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[{ task, ...ctx }]) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[{ task: alias, ...ctx }]) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[...[{ task }]]) => {
  console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[...[{ task: alias }]]) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (...[...[{ task: alias, ...ctx }]]) => {
  console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

//
// it('subtracts', () => {
//   expect(subtract(3, 2)).toBe(1);
// });

// it('multiplies', () => {
//   expect(multiply(3, 3)).toBe(9);
// });
