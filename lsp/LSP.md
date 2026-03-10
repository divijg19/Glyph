# Glyph Language Server (LSP) — Minimal Guidance

This document outlines a minimal, pragmatic approach to editor tooling for Glyph. LSP implementations should remain lightweight and spec-driven; they are optional and must not replicate runtime behavior.

Core responsibilities:

- Syntax validation and basic diagnostics (syntax errors, spec violations, manifest mismatches)
- Symbol navigation (go-to-definition, find references, outline)
- Simple completions for keywords, built-ins, and exported symbols
- Manifest validation (warn when source uses undeclared capabilities)

Design constraints:

- The LSP must not attempt to execute Glyph code or become runtime-aware beyond static spec checks.
- Keep implementations language-agnostic; choice of implementation language is up to contributors.
- Prioritize responsiveness and incremental analysis for edit-time feedback.

Non-goals:

- The LSP should not perform compilation, runtime scheduling, or enforce host policies at edit time.

This minimal LSP guidance ensures useful editor feedback without turning the language server into a runtime or large engineering commitment.