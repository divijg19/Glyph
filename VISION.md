# Glyph - Vision

Glyph is a small, WASM-first, embeddable scripting language designed to be tiny, portable, and capability-safe. It targets lightweight scripting and plugin scenarios such as Rust game engines, plugin systems, automation, and browser UI scripting. Glyph intentionally keeps scope minimal so it can be embedded in hosts with clear capability boundaries.

---

## The Problem `Glyph` Solves

Modern stacks suffer from fragmentation:

* UI logic lives in one language
* Backend plugins live in another
* Configuration is YAML or JSON
* Scripting uses ad-hoc JS, Lua, or unsafe eval
* Plugins are hard to sandbox
* Hot reload is inconsistent across layers

Existing options fail because they are either:

* too large (JavaScript),
* too unsafe (dynamic eval),
* too restrictive (Starlark),
* too low-level (Rust/C++),
* or not portable (platform-locked DSLs).

**`Glyph` is designed specifically to live in the middle.**

---

## Purpose

`Glyph` is intended as:

- A tiny scripting language for UI logic, automation, and game rules
- A plugin language for Rust hosts and other capability-secure runtimes
- A WASM compilation target for portable, sandboxed modules

Glyph is not a general-purpose systems language, VM, or frontend framework.

---

## Design Philosophy

- Complement, don’t compete: host languages (Go, Rust, etc.) remain the place for systems and heavy lifting.
- Minimal surface: few keywords, a compact AST, and a deliberately small standard library.
- WASM as the production contract: compile to WASM, run in host-provided environments.
- Security by design: capabilities and manifests govern side effects.

---

## Development Model

Edit a Glyph source file → compile to WASM using the Rust compiler → hot-reload the module in the host via WASM module replacement.

Hot reload is achieved by swapping WASM modules in the host; Glyph does not rely on a separate interpreter for dev-mode execution.

---

## Language Surface (intent)

Keep the language extremely small. Target values and structures include:

- numbers, strings, booleans
- lists (arrays) and maps (string-keyed)
- functions and modules
- control flow: `if`, `for`, `return`, `break`, `continue`

Keyword set should remain minimal (approx. 10–12 keywords).

---

## Target Architecture

Glyph source → Rust compiler → WASM module → Host runtime (provides capabilities such as DOM, timers, storage, engine hooks).

Glyph itself does not implement a VM, scheduler, GC, or a custom runtime — WASM is the execution runtime.

---

## Non-Goals

Glyph intentionally avoids:

- Interpreter-first or dual-runtime architectures
- JITs, custom VMs, or language-specific schedulers
- Complex type systems, macros, or generics
- Large package registries or sweeping ecosystem ambitions

These directions would contradict Glyph’s goals of smallness and embeddability.

---

## Guiding Principle

Glyph must remain small, portable, and embeddable: useful without being burdensome to host or maintain.
