# [RUST] Fluide

## About Fluide

Fluide (French for “fluid”) is a feather-light CLI that improves the Vite development experience by automating repetitive project setup tasks. It aims to keep configuration smooth, predictable, and out of the way.

By handling package installation, configuration edits, and import management automatically, Fluide reduces setup overhead and helps projects stay clean as they grow.

# Installation

```bash
cargo install --git https://github.com/Vaaris16/fluide --branch main
```

# Package Options

| Package Name  | Category      | Frameworks Supported                                                         | Usage (Bash)                              |
| ------------- | ------------- | ---------------------------------------------------------------------------- | ----------------------------------------- |
| `tailwindcss` | CSS Framework | Vanilla, Vue, React, Preact, Lit, Svelte, Solid, Ember, Qwik, Angular, Marko | `fluide add tailwindcss`                  |
| `sass`        | CSS Framework | Vanilla, Vue, React, Preact, Lit, Svelte, Solid, Ember, Qwik, Angular, Marko | `fluide add sass --framework <framework>` |
| `unocss`      | CSS Framework | Vanilla, Vue, React, Preact, Lit, Svelte, Solid, Ember, Qwik, Angular, Marko | `fluide add unocss`                       |

# Requirments

- Node.js (npm must be available)
- Rust (to build the CLI)
- Existing Vite project
