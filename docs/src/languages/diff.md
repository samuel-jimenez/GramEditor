# Diff

Diff support is available natively in Tehanu.

- Tree-sitter: [zed-industries/the-mikedavis/tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff)

## Configuration

Tehanu will not attempt to format diff files and has [`remove_trailing_whitespace_on_save`](https://tehanu.liten.app/docs/configuring-zed#remove-trailing-whitespace-on-save) and [`ensure-final-newline-on-save`](https://tehanu.liten.app/docs/configuring-zed#ensure-final-newline-on-save) set to false.

Tehanu will automatically recognize files with `patch` and `diff` extensions as Diff files. To recognize other extensions, add them to `file_types` in your Tehanu settings.json:

```json [settings]
  "file_types": {
    "Diff": ["dif"]
  },
```
