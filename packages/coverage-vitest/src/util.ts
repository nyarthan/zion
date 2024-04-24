export function noop(..._args: any[]) {}

// TODO: support other file formats, read from vitest config
export function isTestFile(path: string) {
  return path.endsWith('test.ts') || path.endsWith('test.tsx');
}

export function isSourceFile(path: string) {
  return true;
}
