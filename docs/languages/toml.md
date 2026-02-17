# TOML

TOML support is built into the editor.

- Tree-sitter: [tree-sitter/tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml)

- Crate: [tree-sitter-toml-ng](https://crates.io/crates/tree-sitter-toml-ng)

There is LSP support via [Taplo](https://taplo.tamasfe.dev), but it is disabled
by default. To enable the LSP, add this to your `settings.jsonc`:

```jsonc
{
  "languages": {
    "TOML": {
      "language_servers": ["taplo", "..."],
    },
  },
}
```

