# Glyph - System Architecture

Glyph uses a deliberately small architecture built around one production path.

## Canonical Pipeline

```text
Glyph source
→ Rust compiler
→ WASM module
→ Host runtime
```

This is the architectural center of the project.

## Responsibilities

Compiler responsibilities:

- parse source
- produce a canonical AST
- compile the AST to WASM
- validate manifest and ABI compatibility
- package portable artifacts

Host runtime responsibilities:

- instantiate the WASM module
- expose capability APIs such as DOM access, timers, storage, and engine hooks
- enforce permissions and resource limits
- replace modules during hot reload

Glyph keeps these boundaries sharp so the language stays embeddable.

## Specifications

The specifications under `spec/` define language semantics, AST structure, module manifests, and the host ABI. Tooling and compiler behavior must conform to those documents.

## Editor Tooling

Editor support should stay minimal. Diagnostics, basic navigation, and manifest validation are enough for the core project scope.

## Security Model in Architecture

Modules do not receive ambient authority. Capabilities are declared in the manifest and granted by the host at load time. The host remains responsible for policy decisions and enforcement.

## Repository Mapping

```text
spec/        → language contracts
sparq/       → Rust compiler and packaging
lsp/         → minimal editor tooling guidance
docs/        → architecture and onboarding
examples/    → host embedding examples
```

## Architectural Constraint

If a change increases the project beyond a small embeddable language and compiler, it moves away from Glyph's intended shape.