# Glyph — Module Manifest Specification (Minimal)

> Status: Draft — the manifest declares a module's required capabilities and runtime constraints.

The manifest is the primary mechanism for capability control and safe module deployment.

---

## 1. Purpose

The manifest declares what a module is allowed to do: required capabilities, resource limits, and exports. Hosts use the manifest to validate and enforce permissions at load time.

---

## 2. Format

- Format: JSON (UTF-8)
- Schema: versioned and validated
- Location inside an optional package: `manifest.json`

Top-level fields include `spec_version`, `module`, `capabilities`, `exports`, `resources`, and `integrity`.

---

## 3. Example Structure

```json
{
  "spec_version": "1.0",
  "module": { "name": "example", "version": "0.1.0" },
  "capabilities": [],
  "exports": [],
  "resources": {},
  "integrity": {}
}
```

---

## 4. Module Metadata

`module` metadata contains `name` (kebab-case), `version` (semver), and an optional description.

---

## 5. Capabilities

Capabilities are string identifiers that represent host-provided capability namespaces, e.g. `io.read`, `net.fetch`, `render.draw`.

Rules:

- Capabilities are explicitly granted by the host
- Absence of a capability implies no access

---

## 6. Exports

`exports` lists exported function names that hosts may invoke. Export metadata is declarative; hosts manage invocation semantics.

Example:

```json
"exports": ["init", "handle_event"]
```

---

## 7. Resource Limits

Resource limits are upper bounds enforced by the host, e.g. memory, CPU time, and wall-time.

```json
"resources": { "memory_kb": 1024, "cpu_ms": 200, "wall_ms": 1000 }
```

---

## 8. Integrity & Signing

Optional fields for `source_sha256` and signatures support secure distribution and verification.

---

## 9. Validation

Hosts and build tools must validate that required fields exist, requested capabilities are known, and exported functions match the compiled module.

---

## 10. Relationship to Source

Source code cannot grant capabilities or override manifest-enforced limits. The manifest is authoritative for permissions.

---

## 11. Versioning

`spec_version` governs manifest semantics; incompatible changes require version bumps.

---

## 12. Security Invariant

A Glyph module can only perform the actions its manifest and the host explicitly allow.
