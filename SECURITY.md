# Glyph - Security Model

Glyph security is centered on capability-secure WASM modules running inside host-controlled environments.

## Threat Model

Glyph assumes modules may be untrusted. Hosts must remain safe even when loading hostile or buggy modules.

## Core Rule

Modules can only do what the host explicitly allows.

## Capabilities

Capabilities are:

- declared in the manifest
- granted by the host at load time
- enforced by the host runtime

Common examples include IO, storage, timers, DOM access, and engine hooks.

## Isolation

WASM provides the execution boundary. Modules do not receive direct access to host memory or operating system resources.

## Resource Limits

Hosts should enforce upper bounds for:

- memory
- CPU time
- wall-clock time
- host handles

## Error Containment

Module failures must terminate the module safely and return structured error information to the host.

## Tooling Role

Specs and toolchain validation help catch capability and manifest problems before load time, but the host remains the final enforcement point.

## Invariant

Glyph should remain safe to embed even when the module author is untrusted.