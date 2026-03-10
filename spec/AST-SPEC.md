# Glyph — Canonical AST Specification (Minimal)

> Status: Draft — the AST is intentionally compact and authoritative for compiler and tooling behavior.

This document describes a minimal canonical AST for Glyph. The AST is the authoritative representation of source semantics used by the compiler and tooling.

---

## 1. Purpose of the Canonical AST

The AST decouples language semantics from implementation details and provides a stable contract for the compiler and tools. Implementations (compiler, packager, and tooling) must consume or emit this AST without inventing semantics.

---

## 2. Representation and Goals

The canonical AST should be compact, explicit, and versioned. Implementations may choose a convenient serialization (JSON, protobuf, etc.) for interchange and storage; compatibility is maintained via a `spec_version` field.

Design principles:

- Explicit over implicit
- Minimal node set
- Host-agnostic nodes (no runtime-specific concepts)
- Forward-compatible field versioning

---

## 3. Top-Level Structure

A module AST contains:

- Header
- Imports
- Declarations
- Exports
- Body

One module node per source file.

---

## 4. Core Node Types

Module: `spec_version`, `imports[]`, `declarations[]`, `exports[]`, `body[]`, `source_map`.

Import: `name`, `alias?`, `capability?` (logical host capability reference).

Declaration variants: `FunctionDeclaration`, `VariableDeclaration`.

FunctionDeclaration: `name`, `parameters[]`, `body[]`, `source_span`.

VariableDeclaration: `name`, `initializer` (Expression), `is_mutable` (default false).

---

## 5. Statements

Supported statement nodes:

- `Block`
- `If`
- `For` (iteration)
- `Return`
- `Break`
- `Continue`
- `ExpressionStatement`

Each statement carries a `source_span` for tooling and diagnostics.

---

## 6. Expressions

Expression nodes include:

- `Literal` (number, string, boolean, nil)
- `Identifier`
- `BinaryExpression`
- `UnaryExpression`
- `CallExpression`
- `FunctionExpression`
- `ArrayLiteral`
- `MapLiteral`
- `IndexExpression`
- `MemberAccess`

The core AST stays limited to the minimal language surface described in `LANGUAGE-SPEC.md`.

---

## 7. Literals

Literal node kinds: `NumberLiteral`, `StringLiteral`, `BooleanLiteral`, `NilLiteral`, `ArrayLiteral`, `MapLiteral`.

---

## 8. Control Flow Semantics

Control flow is structural. Example: `IfStatement(condition, then_block, else_block?)`.

Loops are explicit (`For`), and `break`/`continue` are represented as statements.

---

## 9. Source Mapping

Each AST node includes a `source_span { file, start, end }` to support accurate diagnostics and editor tooling.

---

## 10. Versioning & Compatibility

AST includes `spec_version`. New optional fields are allowed; removing or changing fields is a breaking change and must be versioned accordingly.

---

## 11. Forbidden in the AST

The AST must not encode runtime internals, host-specific types, or implicit environment state.

---

## 12. Invariant

If the compiler and tools operate on the same AST, observable behavior and diagnostics should be consistent across the toolchain.
