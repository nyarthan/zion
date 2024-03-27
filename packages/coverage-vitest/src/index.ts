import type { TransformResult } from "vite";
import type {
  AfterSuiteRunMeta,
  CoverageProvider,
  CoverageProviderModule,
  ReportContext,
  ResolvedCoverageOptions,
  Vitest,
} from "vitest";
import { noop } from "./util";

const ProviderModule: CoverageProviderModule = {
  getProvider(): CoverageProvider {
    return new Provider();
  },
};

class Provider implements CoverageProvider {
  public name = "zion";
  private options!: ResolvedCoverageOptions;

  public initialize(ctx: Vitest): void | Promise<void> {
    this.options = { ...ctx.config.coverage, provider: "istanbul" };
  }

  public resolveOptions(): ResolvedCoverageOptions<
    "istanbul" | "v8" | "custom" | undefined
  > {
    return this.options;
  }

  clean(clean?: boolean | undefined): void | Promise<void> {
    noop(clean);
  }

  reportCoverage(
    reportContext?: ReportContext | undefined,
  ): void | Promise<void> {
    noop(reportContext);
  }

  onAfterSuiteRun(meta: AfterSuiteRunMeta): void | Promise<void> {
    noop(meta);
  }

  onFileTransform(
    sourceCode: string,
    id: string,
    pluginCtx: any,
  ):
    | (string | void | Partial<TransformResult> | null | undefined)
    | Promise<string | void | Partial<TransformResult> | null | undefined> {
    noop(sourceCode, id, pluginCtx);
  }
}

export default ProviderModule;
