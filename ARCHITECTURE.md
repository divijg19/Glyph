# **Glyph — System Architecture**


This document defines the **system architecture of Glyph**:
how its parts fit together, why they are separated, and what invariants must *never* be violated.

Glyph is not a single program.
It is a **coherent system** composed of language, runtimes, tooling, and contracts.

---

## 1. Architectural Intent

Glyph exists to add a **missing layer** to modern software stacks:

> A small, safe, portable scripting and module layer that complements—not competes with—Go, Dart, and Flutter.

The architecture is intentionally:

* **Layered**
* **Spec-driven**
# Glyph — System Architecture (Simplified)

This document defines the minimal, production-focused architecture for Glyph: a small, WASM-first scripting language intended to be embedded in capability-secure hosts. The architecture emphasizes a single compilation pipeline, small specifications, and clear host responsibilities.

---

## 1. Architectural Intent

Glyph exists to provide a compact, safe scripting layer for embedding in hosts (game engines, servers, browsers). The architecture is intentionally minimal, spec-driven, and WASM-first.

Key properties:

- Layered and spec-driven
- WASM as the production contract
- Capability-based security
- Small, auditable language surface

---

## 2. Canonical Pipeline

Glyph follows a simple, single pipeline:

```
Glyph source
   ↓
Rust compiler (sparq/)  →  AST → WASM
   ↓
WASM module (.wasm / .rwm)
   ↓
Host runtime (provided by integrator)
```

The host runtime is responsible for providing capability APIs (DOM, timers, storage, engine hooks), enforcing resource limits, and managing module lifecycle. Glyph itself does not implement a VM, scheduler, GC, or JIT.

---

## 3. Specifications (The Source of Truth)

The spec layer defines language semantics, the canonical AST, the module manifest, and the host ABI. It is authoritative: implementations and tools must conform to the specs found under `spec/`.

Non-negotiable rule: specs define semantics; implementations implement them.

---

## 4. Compiler vs Host Responsibilities

Compiler (sparq/):

- Parses source and emits a canonical AST
- Compiles AST to WASM artifacts
- Validates manifests and basic ABI conformance
- Produces reproducible, portable module artifacts

Host runtime (integrator responsibility):

- Loads and instantiates WASM modules
- Provides capability APIs and enforces permissions
- Applies resource quotas and scheduling policies
- Performs hot-reload by module replacement

Glyph intentionally pushes execution semantics to the host; it does not ship an opinionated runtime VM.

---

## 5. Tooling & Editor Support (Minimal)

Tooling should be lightweight and spec-driven. Editor/IDE support (LSP) is optional and should provide minimal features: syntax validation, basic diagnostics, symbol navigation, and manifest validation. LSP must not become a runtime or execution engine.

---

## 6. Module Model & Security

Modules are capability-scoped and isolated. A manifest declares required capabilities and resource limits; hosts grant capabilities at instantiation time. No module gains ambient authority by default.

Invariant: A module can only do what its manifest and host allow.

---

## 7. Artifact Flow (Source → Production)

1. Author Glyph source
2. Compile with `sparq` → WASM
3. Package WASM with manifest (optional `.rwm` wrapper)
4. Host loads WASM and enforces capabilities and quotas

Hot reload is achieved by replacing the module instance in the host.

---

## 8. Repository Mapping

```
spec/        → contracts (truth)
sparq/       → Rust compiler (AST → WASM) and packaging
lsp/         → minimal editor tooling (optional)
docs/        → architecture and guides
examples/    → small host integration examples
```

---

## 9. What Glyph Is Not

Glyph deliberately avoids:

- Dual-runtime or interpreter-first architectures
- Custom VMs, JITs, or in-language schedulers
- Complex type systems, macros, or generics
- Large package registries or extensive ecosystem governance

Staying small and embeddable is the guiding constraint.

---

## 10. Final Invariants

1. Specs define semantics.
2. No ambient authority for modules.
3. WASM is the production contract.
4. Tooling validates but does not weaken security.

If a proposal violates these invariants, it should be rejected.