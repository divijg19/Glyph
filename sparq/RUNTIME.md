# Sparq — Compiler & Packaging (Simplified)

> Status: Draft — Sparq is the Rust-based compiler and packager that emits WASM artifacts for Glyph.

Sparq focuses on compiling the canonical AST to portable WASM modules and validating module manifests and ABI conformance. Execution of WASM modules is the responsibility of the host runtime.

## 1. Role of Sparq

Sparq responsibilities:

- Parse Glyph source and produce a canonical AST
- Compile AST → WASM (portable, reproducible artifacts)
- Validate manifest and ABI compatibility
- Package WASM with manifest metadata (optional `.rwm` wrapper)

Sparq is implemented in Rust and intentionally does not host a full runtime VM — execution is delegated to the host.

---

## 2. Artifact Lifecycle

1. Build: source → AST → WASM
2. Package: WASM + manifest + integrity metadata → artifact
3. Distribute: signed or unsigned artifact
4. Host: load artifact, validate manifest, instantiate WASM

Sparq ensures artifacts are reproducible and spec-compliant.

---

## 3. Validation Rules

Before packaging, Sparq performs static validation:

- Manifest spec version and schema validation
- Export signatures vs compiled exports
- ABI compatibility checks
- Integrity metadata generation

Failure to pass validation prevents artifact creation.

---

## 4. Security & Capabilities

Sparq validates capability declarations and embeds manifest metadata, but enforcement of capabilities is performed by the host at instantiation time.

---

## 5. Host Execution

Hosts load and execute WASM modules produced by Sparq. Hosts are responsible for:

- Binding capability namespaces
- Enforcing resource quotas
- Managing module lifecycle and hot-reload
- Providing any scheduling or async behavior

Sparq's role ends at producing validated, portable modules.

---

## 6. Observability & Testing

Sparq should support golden compiler tests (AST → WASM → outputs) and provide helpful diagnostics for compilation and packaging failures.

---

## 7. Non-Goals

Sparq does not:

- Implement a production VM, scheduler, or garbage collector
- Provide opinionated host runtime features
- Manage package registries or complex distribution protocols (these are out-of-scope for the core compiler)

The project prefers small, composable tools over a large, integrated runtime.