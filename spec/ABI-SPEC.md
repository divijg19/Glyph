# Glyph — Host ABI Specification (Simplified)

> Status: Draft — the ABI defines a small, explicit boundary between compiled Glyph modules and host environments.

This document specifies how data and control flow across the WASM-module / host boundary. The ABI intentionally keeps the surface minimal and host-agnostic.

---

## 1. Purpose

The ABI enables safe, predictable communication between a Glyph WASM module and its host. It supports WASM-first execution and ensures host-controlled effects and clear memory ownership.

---

## 2. Design Principles

1. Minimal surface area — only essential primitives supported
2. Explicit ownership and lifetimes
3. Host-controlled effects — modules cannot perform effects without host mediation
4. Language-agnostic — ABI does not expose host language internals

---

## 3. Execution Context

Each module instance runs with a host-provided execution context containing:

- capability table
- resource limits
- host function registry
- error handling hooks

This context is provided at instantiation by the host.

---

## 4. Supported Value Types

Core ABI value types:

| ABI Type | Description |
| -------- | ----------- |
| `i64`    | 64-bit signed integer |
| `f64`    | 64-bit floating point |
| `bool`   | Boolean |
| `string` | UTF-8 encoded string |
| `bytes`  | Raw byte buffer |
| `array`  | Ordered list of values |
| `map`    | String-keyed associative map |
| `nil`    | Absence of value |
| `handle` | Opaque host-managed reference |
| `error`  | Structured error object |

Only these types cross the module boundary.

---

## 5. Strings & Bytes

Strings are UTF-8. Bytes are raw buffers; zero-copy semantics are host-dependent and must be declared.

---

## 6. Arrays & Maps

Arrays and maps are passed as references or serialized values per host ABI conventions. Maps use string keys.

---

## 7. Handles

Handles represent host-managed objects (files, sockets, UI widgets, textures, etc.). Modules cannot introspect handles; hosts may revoke handles.

---

## 8. Host Function Calls

Modules invoke host functions by name within capability namespaces, e.g. `io.read(path) -> bytes`.

Rules:

- Calls fail if the capability is not granted
- Arguments are validated at the boundary
- Hosts control scheduling and any asynchronous behavior

The ABI does not mandate a particular async model; hosts may expose asynchronous patterns via handles or out-of-band conventions.

---

## 9. Error Model

Errors are structured values that propagate to the host. Hosts may convert them into native exceptions or error codes.

---

## 10. Memory Model

WASM linear memory is the default. Modules cannot access host memory directly unless a host explicitly exposes shared memory regions with clear lifetime rules.

---

## 11. Determinism

When a host enables deterministic execution, time, randomness, and IO are virtualized by the host. The language itself does not provide a scheduler.

---

## 12. Versioning & Compatibility

ABI is versioned; breaking changes increment the major version. Hosts and compilers must declare supported ABI versions.

---

## 13. Forbidden Behaviors

The ABI must never permit arbitrary memory access, host stack manipulation, or unmediated filesystem/network access.

---

## 14. Testing

ABI conformance tests are recommended to ensure portability across hosts.

---

## 15. Invariant

If two hosts implement this ABI correctly, a module should exhibit consistent observable behavior.
