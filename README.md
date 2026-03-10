# Glyph

A tiny Go-like scripting language that compiles to WASM, enabling portable sandboxed behavior modules in capability-secure hosts.

Glyph is small, embeddable, and WASM-first. It targets use cases such as Rust game engines, plugin systems, automation, and browser UI scripting. Glyph is complementary to Go + Templ + HTMX stacks and is intentionally minimal — not a VM, JIT, or large ecosystem language.

---

## Key Principles

- Small and embeddable: suitable for UI logic, game rules, and plugins.
- WASM-first: production artifacts are portable WASM modules.
- Capability-secure: hosts explicitly grant access to IO, timers, storage, and engine hooks.
- Minimal language surface: focus on a concise scripting DSL.

---

## Project Structure (simplified)

```
spec/        → Language, AST, manifest, ABI specifications
sparq/       → Rust compiler (AST → WASM) and packaging
lsp/         → Minimal language server (optional)
examples/    → Integration examples (hosts, web)
docs/        → Guides and architecture
```

---

## Getting Started (developer preview)

Clone the repository:

```bash
git clone https://github.com/divijg19/glyph
cd glyph
```

Build the Rust compiler or WASM artifacts using standard Rust tooling (examples and scripts in `sparq/`).

Hot reload in development is intended to work via WASM module replacement: edit a glyph file → compile to WASM → swap module in the host.

---

## Roadmap (high level)

- Establish a small, stable language spec and canonical AST
- Implement a Rust-based compiler that emits WASM modules
- Provide minimal host integration examples (Rust hosts, web)
- Offer basic editor tooling (minimal LSP) and examples
- Harden security (capabilities, quotas) and stabilize v1

More details in `VISION.md` and `ARCHITECTURE.md`.

---

## Contributing

Please see `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md` for contribution guidelines.

---

## License

See `LICENSE`.
