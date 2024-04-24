import { expect, it } from 'vitest';

import { add } from './math';

// prettier-ignore-start
it('adds ()', (ctx) => {
  // console.log(ctx.task);
  expect(add(1, 2)).toBe(3);
});

it('adds (default value)', (ctx = 1 as any) => {
  // console.log(ctx.task);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured)', ({ task }) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (default value, object destructured)', ({ task } = 1 as any) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured, task default value)', ({ task = 1 as any }) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured, task aliased)', ({ task: alias }) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured, fully spread)', ({ ...ctx }) => {
  // console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured, partially spread)', ({ task, ...ctx }) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured, partially spread, task aliased)', ({ task: alias, ...ctx }) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured)', (...[ctx]) => {
  // console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, ctx default value)', (...[ctx = 1 as any]) => {
  // console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured)', (...[{ task }]) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured, task aliased)', (...[
  { task: alias },
]) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured, fully spread)', (...[
  { ...ctx },
]) => {
  // console.debug(ctx.task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured, partially spread)', (...[
  { task, ...ctx },
]) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured, partially spread, task aliased)', (...[
  { task: alias, ...ctx },
]) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured twice, object destructured)', (...[...[{ task }]]) => {
  // console.debug(task.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured twice, object destructured, task aliased)', (...[
  ...[{ task: alias }]
]) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured twice, object destructured, partially spread, task aliased)', (...[
  ...[{ task: alias, ...ctx }]
]) => {
  // console.debug(alias.id);
  expect(add(1, 2)).toBe(3);
});

it('adds (object destructured 2 levels)', ({ task: { id } }) => {
  // console.debug(id);
  expect(add(1, 2)).toBe(3);
});

// @ts-expect-error don't care - this is valid
it('adds (object destructured 2 levels, 2. arg)', ({ task: { id } }, other) => {
  // console.debug(id);
  expect(add(1, 2)).toBe(3);
});

it('adds (args spread, array destructured, object destructured 2 levels)', (...[
  {
    task: { id },
  },
]) => {
  // console.debug(id);
  expect(add(1, 2)).toBe(3);
});

// @ts-expect-error don't care - this is valid
it('adds (args spread, array destructured, object destructured 2 levels, 2. array member)', (...[
  {
    task: { id },
  },
  other,
]) => {
  // console.debug(id);
  expect(add(1, 2)).toBe(3);
});

it('adds ()', (ctx) => {
  expect(add(1, 2)).toBe(3);
});

it.concurrent('adds (concurrent)', () => {
  expect(add(1, 2)).toBe(3);
});

it.sequential('adds (sequential)', () => {
  expect(add(1, 2)).toBe(3);
});

/* it.only('adds (only)', () => {
  expect(add(1, 2)).toBe(3);
}); */

it.skip('adds (skip)', () => {
  expect(add(1, 2)).toBe(3);
});

it.todo('adds (todo)', () => {
  expect(add(1, 2)).toBe(3);
});

it.fails('adds (fails)', () => {
  expect(add(1, 2)).toBe(4);
});

it.each([
  [1, 2, 3],
  [4, 5, 9],
])('adds (each)', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

it.runIf(true)('adds (runIf)', () => {
  expect(add(1, 2)).toBe(3);
});

it.skipIf(true)('adds (skipIf)', () => {
  expect(add(1, 2)).toBe(3);
});

it.concurrent.concurrent('adds (concurrent, concurrent)', () => {
  expect(add(1, 2)).toBe(3);
});

it.concurrent.each([
  [1, 2, 3],
  [4, 5, 9],
])('adds (concurrent, each)', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

it.runIf(true).each([
  [1, 2, 3],
  [4, 5, 9],
])('adds (runIf, each)', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});

it.runIf(true).concurrent.skip.each([
  [1, 2, 3],
  [4, 5, 9],
])('adds (runIf, concurrent, skip, each)', (a, b, c) => {
  expect(add(a, b)).toBe(c);
});
// prettier-ignore-end
