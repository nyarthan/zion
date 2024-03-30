import { z } from 'zod';

export const locSchema = z.object({
  start: z.number(),
  end: z.number(),
});

export const stmtSchema = z.object({
  start: z.number(),
  end: z.number(),
});

export const fnSchema = z.object({
  name: z.string().optional(),
  decl: locSchema,
  loc: locSchema,
});

export const branchType = z.enum(['if', 'else']);

export const branchSchema = z.object({
  type: branchType,
  loc: locSchema,
});

export const coverageDataSchema = z.object({
  stmts: stmtSchema.array(),
  fns: fnSchema.array(),
  branches: branchSchema.array(),
});
