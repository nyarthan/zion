import * as fs from 'node:fs';
import { createRequire } from 'node:module';
import * as path from 'node:path';

const require = createRequire(import.meta.url);

const outDir = path.join(process.cwd(), 'schemas');

const coverageDataSchemaPath = require.resolve('@zion/typegen/coverage-data');

if (!fs.existsSync(outDir)) {
  fs.mkdirSync(outDir);
}

fs.copyFileSync(coverageDataSchemaPath, path.join(outDir, 'coverage-data.json'));
