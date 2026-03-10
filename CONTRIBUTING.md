# Contributing to Glyph

Glyph is intentionally small. Contributions should preserve that.

## Project Priorities

Contributions should support:

- clear specifications
- small composable features
- secure host embedding
- maintainable compiler and tooling code

## Good Areas for Contribution

- language and spec clarifications
- AST, ABI, and manifest work
- Rust compiler improvements in `sparq/`
- minimal editor tooling
- examples and documentation
- tests and security validation

## Spec-First Rule

If a change affects semantics, compatibility, or security, update the relevant file in `spec/`.

## Development Workflow

1. Make a focused change.
2. Add or update tests where appropriate.
3. Keep docs in sync with the implementation and scope.
4. Open a pull request.

## Coding Standards

- prefer clarity over cleverness
- keep public surface area small
- avoid changes that expand project scope without discussion

## Review Criteria

Changes are reviewed for:

- correctness
- spec alignment
- security impact
- scope discipline

## Invariant

No contribution should push Glyph away from being a small embeddable WASM-first scripting language.