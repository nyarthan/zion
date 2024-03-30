import * as fs from 'node:fs';
import * as path from 'node:path';

import { zodToJsonSchema } from 'zod-to-json-schema';

import { coverageDataSchema } from '../src/schemas';

const outDir = path.join(process.cwd(), 'dist');

if (!fs.existsSync(outDir)) {
  fs.mkdirSync(outDir);
}

const jsonSchema = zodToJsonSchema(coverageDataSchema, {
  name: 'coverageData',
  $refStrategy: 'none',
});

fs.writeFileSync(path.join(outDir, 'coverage-data.json'), JSON.stringify(jsonSchema));
