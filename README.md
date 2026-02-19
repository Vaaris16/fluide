# [RUST] Fluide

## About Fluide

Fluide is a CLI tool that handles the boring, repetitive work of adding popular packages to Vite projects.
No more manual installs, config edits, or duplicate imports — Fluide does it for you.

# Installation

```bash
cargo install --git https://github.com/Vaaris16/fluide --branch main
```

# Usage

```bash
fluide add [package options]
```

# Package Options

| Package Name  | Category      | Frameworks Supported                                                         | Usage (Bash)                              |
| ------------- | ------------- | ---------------------------------------------------------------------------- | ----------------------------------------- |
| `tailwindcss` | CSS Framework | Vanilla, Vue, React, Preact, Lit, Svelte, Solid, Ember, Qwik, Angular, Marko | `fluide add tailwindcss`                  |
| `sass`        | CSS Framework | Vanilla, Vue, React, Preact, Lit, Svelte, Solid, Ember, Qwik, Angular, Marko | `fluide add sass --framework <framework>` |

# Requirments

- Node.js (npm must be available)
- Rust (to build the CLI)
- Existing Vite project
