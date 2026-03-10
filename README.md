# Glyph

Glyph is a lightweight WASM-first scripting language designed to act as a modern Lua-like runtime for Rust systems and web scripting.

A tiny Go-like scripting language that compiles to WASM and runs in capability-secure hosts.

Glyph is built for small embeddable scripting tasks: Rust game engines, plugin systems, automation, browser UI scripting, and Go + Templ + HTMX applications that need a compact behavior layer.

## Architecture

Glyph follows a single production path:

```text
Glyph source
→ Rust compiler
→ WASM module
→ Host runtime
```

The host runtime provides capabilities such as DOM access, timers, storage, and engine hooks. Glyph stays small by treating WASM as the execution runtime instead of building its own execution environment.

## Language Scope

Glyph is intentionally small. The core language centers on:

- numbers
- strings
- booleans
- lists
- maps
- functions
- modules
- control flow through `if`, `for`, `return`, `break`, and `continue`

The keyword set is intentionally minimal.

## Project Structure

```text
spec/        → language, AST, manifest, and ABI specifications
sparq/       → Rust compiler and WASM packaging
lsp/         → minimal editor tooling guidance
docs/        → architecture and onboarding guides
examples/    → host integration examples
```

## Development Model

Development should stay simple:

```text
edit glyph file
→ compile to wasm
→ hot reload module
```

Hot reload is performed by replacing the compiled WASM module in the host.

## Status

Glyph is intentionally scoped to remain small, portable, and achievable. The target shape is a focused compiler and spec set, not a large language platform.

See `VISION.md`, `ARCHITECTURE.md`, and `spec/` for the detailed project direction.

## Contributing

See `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.

## License

See `LICENSE`.
