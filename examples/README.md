# Glyph - Examples

Examples show how Glyph modules are embedded into hosts.

The examples in this repository should converge on one model:

```text
glyph source
→ wasm module
→ host integration
```

## Purpose

Examples should demonstrate:

- host-module boundaries
- manifest-driven capabilities
- small focused behavior modules
- realistic embedding patterns

## Example Areas

Current directories cover several host shapes already present in the repository:

- `game-engines/` for gameplay scripting and engine hooks
- `go-server/` for plugin-style server extensions
- `templ-htmx/` for server-rendered UI behavior
- `shared/` for common fixtures and helpers

Other example directories should be interpreted through the same WASM-first embedding model.

## Quality Bar

Examples should:

- stay small
- declare required capabilities clearly
- avoid architecture that depends on a separate execution path
- match the current specs

If an example and a spec disagree, the spec wins.