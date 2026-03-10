# Glyph - Vision

Glyph is a lightweight WASM-first scripting language designed to act as a modern Lua-like runtime for Rust systems and web scripting.

Its role is narrow and intentional: provide a small, embeddable scripting layer with Go-like syntax and capability-secure host integration.

## Canonical Scope

Glyph is for:

- Rust game engines
- plugin systems
- automation
- browser UI scripting
- Go + Templ + HTMX applications that need small behavior modules

Glyph should remain small, portable, and embeddable.

## Core Model

Glyph follows one architecture:

```text
Glyph source
→ Rust compiler
→ WASM module
→ Host runtime
```

The host runtime owns execution and exposes capability APIs such as DOM access, timers, storage, and engine hooks.

## Design Direction

Glyph stays useful by staying small:

- a compact language surface
- a single compiler pipeline
- a small spec set
- host-controlled effects through capabilities
- fast development through WASM rebuild and module replacement

## Language Shape

Target features:

- numbers
- strings
- booleans
- lists
- maps
- functions
- modules
- control flow with `if`, `for`, `return`, `break`, and `continue`

The keyword set should stay close to 10–12 keywords.

## Development Model

The intended workflow is:

```text
edit glyph file
→ compile to wasm
→ hot reload module
```

Hot reload happens by replacing the compiled WASM module in the host.

## Boundaries

Glyph is not trying to become a large general-purpose platform. If a feature expands implementation size or runtime surface beyond an embeddable scripting language, it is likely outside scope.

## Guiding Principle

Glyph should add flexibility to a host without adding architectural weight.
