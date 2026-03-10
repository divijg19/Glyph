# Development Workflow Notes

Glyph development should use the same fundamental path as production:

```text
edit source
→ compile to wasm
→ replace module in the host
```

## Goals

- short rebuild cycles
- host-driven hot reload
- behavior that stays aligned with the compiled output

## Hot Reload

Hot reload is done by replacing the compiled module in the host. Hosts may preserve state where it is safe to do so.

## Debugging

Development tooling should emphasize fast rebuilds, source-mapped diagnostics, and simple inspection of module state.