# Zion Coverage Instrumentation

An test instrumentation tool for TypeScript, that not only tracks how often something is covered but also what test caused the coverage.

This is not a fully implemented instrumentation tool, just a POC to see if contextual coverage is viable.

## Setup

- Install rust with the `wasm32-wasi` target

```sh
rustup toolchain install stable

rustup target add wasm32-wasi
```

- Install node dependencies

```sh
pnpm i
```

- Build the coverage data schemas

```sh
pnpm -F @zion/typegen build
pnpm -F @zion/typegen-rs build
```

- Build other packages

```sh
pnpm build
```

- Run example tests

```sh
pnpm -F @zion/example-simple test:coverage
```
