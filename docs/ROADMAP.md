# Glyph — Roadmap (Simplified)

This roadmap focuses on a compact, achievable path to a WASM-first, embeddable scripting language.

## Guiding Principles

- Spec-first: define a small, stable spec before expanding features
- Dev experience: iterate on fast build/reload workflows using WASM module replacement
- Security-first: capability-based manifests and host-enforced quotas

## Phase 0 — Foundation

Goal: establish core specs and a minimal compiler.

Deliverables:

- Canonical specs: `LANGUAGE-SPEC.md`, `AST-SPEC.md`, `MANIFEST-SPEC.md`, `ABI-SPEC.md`
- `sparq/`: Rust compiler that emits WASM
- Basic examples showing embedding in a Rust host and in the browser

Exit criteria: clear, versioned specs and reproducible WASM artifacts.

## Phase 1 — Minimal Tooling & Examples

Goal: make Glyph practical to embed and iterate with.

Deliverables:

- Fast compile & simple hot-reload workflow (WASM module replacement)
- Small example hosts (Rust host, browser wrapper)
- Minimal editor support (lightweight LSP with syntax/manifest checks)
- Test suite for core language features

Exit criteria: developers can compile, load, and iterate on modules easily.

## Phase 2 — Security & Stabilization

Goal: harden runtime interactions and stabilize the v1 spec.

Deliverables:

- Capability enforcement guidance for hosts
- Resource quota recommendations and tests
- Golden tests for language semantics
- Documentation and migration notes

Exit criteria: clear v1 promise and stable core behavior.

## Non-Goals (explicit)

Glyph will not, in core, pursue:

- Interpreter-first dual-runtime architectures
- JITs, macros, generics, or complex type systems
- Large package registries or centralized ecosystem control

These would contradict the small embeddable scope.

---

Further details live in `VISION.md` and the `spec/` directory.