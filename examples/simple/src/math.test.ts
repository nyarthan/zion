import { expect, it } from 'vitest';

import { add } from './math';

it('adds', (ctx) => {
  console.log(ctx.task);
  expect(add(1, 2)).toBe(3);
});

it('adds', (ctx = 1 as any) => {
  console.log(ctx.task);
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

it('adds', ({ task: { id } }) => {
  console.debug(id);
  expect(add(1, 2)).toBe(3);
});

it('adds', ({ task: { id } }, other) => {
  console.debug(id);
  expect(add(1, 2)).toBe(3);
});

// prettier-ignore
it('adds', (...[{task: {id}}]) => {
  console.debug(id);
  expect(add(1, 2)).toBe(3);
});

// prettier-ignore
it('adds', (...[{task: {id}}, other]) => {
  console.debug(id);
  expect(add(1, 2)).toBe(3);
});

it('adds', (ctx) => {
  expect(add(1, 2)).toBe(3);
});

it.concurrent('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.sequential('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.only('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.skip('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.todo('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.fails('adds', () => {
  expect(add(1, 2)).toBe(3);
});

// prettier-ignore
it.each([[1, 2, 3], [4, 5, 9]])('adds', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

it.runIf(true)('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.skipIf(true)('adds', () => {
  expect(add(1, 2)).toBe(3);
});

it.concurrent.concurrent('adds', () => {
  expect(add(1, 2)).toBe(3);
});

// prettier-ignore
it.concurrent.each([[1, 2, 3], [4, 5, 9]])('adds', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

// prettier-ignore
it.runIf(true).each([[1, 2, 3], [4, 5, 9]])('adds', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

// prettier-ignore
it.runIf(true).concurrent.skip.each([[1, 2, 3], [4, 5, 9]])('adds', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});
