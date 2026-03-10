# Development Workflow (replaces historical dev interpreters)

> Note: the repository previously contained a separate Dart-based development interpreter. Glyph's canonical approach is now WASM-first; development workflows should favor fast compilation to WASM and host-driven module replacement.

## Purpose

This document describes the recommended development workflow: fast iteration via quick WASM builds and hot-reloading modules in the host. Glyph does not depend on a separate interpreter for dev-mode execution.

## Recommended Dev Flow

1. Edit Glyph source
2. Compile with the Rust compiler (`sparq`) to a WASM module
3. Reload the module in a running host (hot-replace the WASM instance)

Hosts may provide convenience tooling (file watchers, small runtimes) to streamline the cycle, but such tooling should still use the WASM artifacts and respect manifest capabilities.

## REPL & Debugging

REPLs or quick-eval tools are allowed for developer convenience, but they must not diverge from the canonical compiler semantics. Hosts should offer source-mapped errors and simple inspection hooks.

## Hot Reload Guarantees

Hot reload by module replacement is a best-effort developer feature. Hosts decide whether to preserve state or fully restart modules when swapping WASM instances.

## Deprecated: Dart-based Dev Interpreter

Any historical Dart-based interpreter or tooling in the tree is considered deprecated under the simplified WASM-first scope. Focus new development on the Rust-based compilation and WASM module workflow.