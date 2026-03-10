# Glyph - Language Specification

> Status: Draft

Glyph is a small embeddable scripting language with a deliberately narrow core.

## Scope

Glyph programs are modules compiled to WASM and loaded by hosts. The language is intended for small scripting tasks, plugin logic, automation, and browser-side behavior.

## Core Values

Glyph values are:

- simplicity
- readability
- predictable execution
- portable WASM output
- capability-controlled side effects

## Program Model

A Glyph source file defines a module. A module can contain imports, declarations, exports, and executable statements.

Example:

```glyph
import io

export fn main() {
  let message = "Hello, Glyph"
  io.log(message)
}
```

## Keywords

The keyword set should stay small. A representative set is:

```text
fn
let
if
else
for
return
break
continue
import
export
true
false
nil
```

## Values

Glyph values include:

- numbers
- strings
- booleans
- nil
- lists
- maps
- functions

Glyph is dynamically typed.

## Variables

Variables are declared with `let` and use block scope.

## Functions

Functions are first-class values and are declared with `fn`. Exported entry points are declared with `export fn`.

## Control Flow

Control flow is intentionally limited to:

- `if`
- `for`
- `return`
- `break`
- `continue`

## Imports and Capabilities

Imports refer to host-provided namespaces. Capabilities are declared in the module manifest and granted by the host.

## Errors

Errors propagate to the host when unhandled.

## Execution Semantics

- module execution is isolated
- effects go through host capabilities
- hosts decide resource limits and execution policy

## Relationship to Other Specs

AST structure, ABI details, and manifest rules are defined in `AST-SPEC.md`, `ABI-SPEC.md`, and `MANIFEST-SPEC.md`.

## Rule

If behavior is not specified here, implementations must not invent it.
