import { createRequire } from 'node:module';

import { Options, transform } from '@swc/core';

const require = createRequire(import.meta.url);
const pluginPath = require.resolve('@zion/swc-plugin');

console.debug({ pluginPath });

const options: Options = {
  jsc: {
    parser: {
      syntax: 'ecmascript',
      jsx: true,
    },
    experimental: {
      plugins: [[pluginPath, {}]],
    },
  },
};

export async function instrument(source: string) {
  return transform(source, options);
}
