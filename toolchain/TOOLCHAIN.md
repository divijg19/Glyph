# Glyph Toolchain

This document describes the minimal toolchain shape for Glyph.

## Role

The toolchain exists to:

- compile Glyph source to WASM
- validate manifests and ABI compatibility
- package artifacts for host loading
- inspect module metadata during development

The toolchain does not define language semantics.

## Build Model

The expected build path is:

```text
source
→ AST
→ WASM
→ packaged artifact
```

## Command Shape

Illustrative commands:

```bash
glyph build
glyph inspect
glyph test
```

If the project keeps `rig` integration, it should remain a thin wrapper around the same compiler and packaging flow.

## Validation

The toolchain validates:

- language and manifest versions
- AST structure
- export declarations
- capability declarations
- ABI compatibility

## Packaging

Artifacts may bundle the compiled WASM module with a manifest and integrity metadata. Packaging should remain deterministic.

## Constraint

Tooling should remain small and focused on the compile-and-load workflow rather than growing into a large orchestration layer.