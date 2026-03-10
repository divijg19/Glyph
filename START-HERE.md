# Getting Started with Glyph

Glyph is a small WASM-first scripting language. The intended workflow is simple:

```text
write glyph source
→ compile to wasm
→ load in a capability-secure host
```

## 1. Prerequisites

For the current project direction, the important dependency is Rust for the compiler work in `sparq/`. Additional host tooling depends on the integration example you are using.

## 2. Clone the Repository

```bash
git clone https://github.com/divijg19/glyph
cd glyph
```

## 3. Write a Small Module

Create `hello.gly`:

```glyph
import io

export fn main() {
  io.log("Hello, Glyph")
}
```

## 4. Compile to WASM

Glyph source is intended to compile through the Rust compiler into a WASM module. Exact command names may evolve while implementation catches up, but the architectural flow is fixed:

```text
hello.gly
→ compiler
→ hello.wasm
```

## 5. Load in a Host

The host runtime loads the compiled module, grants capabilities, and invokes exports such as `main`.

## 6. Hot Reload

During development, change the source, rebuild the module, and replace the WASM module in the host.

## 7. Project Layout

```text
glyph/spec/        → authoritative language and ABI docs
glyph/sparq/       → Rust compiler and packaging
lsp/         → minimal editor tooling guidance
docs/        → onboarding and architecture
examples/    → host integration examples
```

## 8. Next Reading

- `spec/LANGUAGE-SPEC.md`
- `ARCHITECTURE.md`
- `VISION.md`
- `examples/README.md`