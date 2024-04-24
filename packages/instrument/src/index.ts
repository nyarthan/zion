import { createRequire } from 'node:module';

import { transform } from '@swc/core';

const require = createRequire(import.meta.url);
const pluginPath = require.resolve('@zion/swc-plugin');

export type InstrumentOptions = {
  type: 'source' | 'test';
};

export async function instrument(source: string, options: InstrumentOptions) {
  return transform(source, {
    jsc: {
      parser: { syntax: 'ecmascript', jsx: true },
      target: 'esnext',
      experimental: { plugins: [[pluginPath, { type: options.type }]] },
    },
  });
}
