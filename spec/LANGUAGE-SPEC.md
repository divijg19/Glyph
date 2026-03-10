# **Glyph — Language Specification**

> **Status:** Draft (authoritative; implementations must conform)

This document defines the **syntax, semantics, and core concepts** of the Glyph language.
It is the **canonical definition** of what Glyph programs mean.

All implementations (Wisp, Sparq, tooling, LSP) must follow this specification.

---

## 1. Scope of the Language

Glyph is a **small, embeddable scripting and module language** designed for:

* UI scripting
* Application plugins
* Game logic
* Server-side extensions
* Configuration and automation
* WASM-based portable modules

Glyph is **not** a systems language, and **not** a replacement for Go, Rust, or Dart.

---

## 2. Design Goals

Glyph prioritizes:

* Simplicity and readability
* Predictable execution
* Sandboxed, capability-based effects
* WASM portability
* Excellent developer experience

Glyph intentionally avoids:

* Implicit global state
* Ambient authority
* Hidden runtime behavior
* Large standard libraries

---

## 3. Program Structure

A Glyph program is a **module**.

A module consists of:

* Declarations
* Imports
* Exports
* Executable statements
* Optional metadata (via manifest, not source)

Example:

```glyph
# Glyph — Language Specification (Minimal)

> Status: Draft — the language spec is intentionally small and authoritative for compiler and tooling behavior.

This document captures the minimal language surface for Glyph: a compact scripting DSL that compiles to WASM and is intended for embedding in capability-secure hosts.

---

## 1. Scope

Glyph is a tiny, embeddable scripting language aimed at:

- UI scripting and browser-side behavior (via WASM)
- Plugin logic for Rust hosts and game engines
- Simple automation and configuration tasks

Glyph is not a full systems language or a VM — it targets small, auditable modules.

---

## 2. Design Goals

- Keep the syntax and semantics minimal and easy to implement
- Ensure portability by targeting WASM
- Enforce capability-based side effects via host manifests
- Avoid complex type systems, macros, or advanced language features

---

## 3. Program Model

A Glyph program is a module. Modules contain imports, declarations, and exports. Hosts decide which exported functions to invoke.

Example:

```glyph
import io

export fn main() {
  let x = 42
  io.log(x)
}
```

---

## 4. Lexical Elements

Identifiers: start with a letter or `_`, may contain letters, digits, and `_`, and are case-sensitive.

Literals: numbers (64-bit float), strings (UTF-8), booleans, `nil`, arrays, and maps.

---

## 5. Keywords

Glyph keeps keywords to a minimum. An initial set:

```
fn  let  export  import
if  else for  return
break  continue
true  false  nil
```

This set is intentionally small (≈10–12 keywords).

---

## 6. Values & Types

Glyph is dynamically typed. Core runtime values:

- number, string, boolean, nil
- list (array), map (string-keyed)
- function

Type errors are runtime errors.

---

## 7. Variables

Variables are declared with `let` and are block-scoped. They are immutable by default.

```glyph
let x = 10
```

Shadowing is permitted.

---

## 8. Functions

Functions are first-class and declared with `fn`.

```glyph
fn add(a, b) { return a + b }
let sq = fn(x) { x * x }
```

Exported functions are declared with `export fn`.

---

## 9. Control Flow

Glyph supports only a small set of control constructs:

- `if` / `else`
- `for` (iteration over arrays or ranges)
- `return`
- `break` / `continue` inside loops

Examples:

```glyph
if x > 0 { ... } else { ... }
for item in items { ... }
```

There is no `while` loop, no `async/await`, and no built-in concurrency model in the core language. Hosts provide any needed scheduling or async behavior.

---

## 10. Imports & Host Capabilities

Imports refer to host-provided capability namespaces. The module manifest declares capabilities; hosts grant them at load time. Source code cannot grant itself capabilities.

```glyph
import io
io.log("hi")
```

---

## 11. Errors

Errors are values that propagate to the host when unhandled. Hosts decide how to map or present errors.

---

## 12. Execution Semantics

- Single-threaded execution per module instance (from the language perspective)
- No shared mutable state between modules
- All effects go through host capabilities

Determinism is a host-provided mode; the language does not define a scheduling model.

---

## 13. Relationship to Other Specs

This document defines language semantics. The AST, ABI, and manifest are specified in `AST-SPEC.md`, `ABI-SPEC.md`, and `MANIFEST-SPEC.md` respectively.

---

## 14. Guiding Rule

If behavior is not specified here, it is undefined. Implementations must not invent semantics beyond the canonical specs.


Glyph supports **async functions** and `await`.
