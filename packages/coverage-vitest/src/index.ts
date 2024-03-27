import type { TransformResult } from 'vite';
import type {
  AfterSuiteRunMeta,
  CoverageProvider,
  CoverageProviderModule,
  ReportContext,
  ResolvedCoverageOptions,
  Vitest,
} from 'vitest';

import { instrument } from '@zion/instrument';

import { noop } from './util';

type GlobalCoverage = Record<string, unknown>;

declare global {
  namespace globalThis {
    var __ZION_COVERAGE__: GlobalCoverage;
  }
}

const ProviderModule: CoverageProviderModule = {
  getProvider(): CoverageProvider {
    return new Provider();
  },

  startCoverage() {
    globalThis.__ZION_COVERAGE__ ||= {};
  },

  takeCoverage() {
    return global.__ZION_COVERAGE__;
  },
};

class Provider implements CoverageProvider {
  public name = '@zion/coverage-vitest';
  private options!: ResolvedCoverageOptions;

  public initialize(ctx: Vitest): void | Promise<void> {
    this.options = ctx.config.coverage;
  }

  public resolveOptions(): ResolvedCoverageOptions {
    return this.options;
  }

  clean(clean?: boolean | undefined): void | Promise<void> {
    noop(clean);
  }

  reportCoverage(reportContext?: ReportContext | undefined): void | Promise<void> {
    noop(reportContext);
  }

  onAfterSuiteRun(meta: AfterSuiteRunMeta): void | Promise<void> {
    const coverage = meta.coverage as GlobalCoverage;
    console.debug(coverage);
  }

  async onFileTransform(
    sourceCode: string,
    id: string,
    pluginCtx: any,
  ): Promise<string | void | Partial<TransformResult> | null | undefined> {
    if (id.includes('math.ts')) {
      const { code } = await instrument(sourceCode);
      return { code };
    }
    noop(sourceCode, id, pluginCtx);
  }
}

export default ProviderModule;
