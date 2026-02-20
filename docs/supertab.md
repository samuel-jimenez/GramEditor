# SuperTab

Inspired by the Vim
[SuperTab](https://www.vim.org/scripts/script.php?script_id=1643) plugin, this
feature makes the Tab key context-sensitive so that the same key can indent,
produce a literal tab character and perform completion, all in one.

Note: The feature is experimental and still a work-in-progress, so it may not
work as smoothly as intended in all scenarios.

The basic concept is that it's possible to figure out what to do when the user
presses tab based on the current context. If the cursor is at the end of some
text, activating SuperTab will open the completion menu. If the cursor is at the
beginning of the line or only preceded by whitespace, it will indent the line.
In any other scenario, it will produce a tab character.

To enable SuperTab, bind the tab key to {#action editor::SuperTab} in your
`keymap.jsonc`:

```jsonc
{
  "context": "Editor",
  "use_key_equivalents": true,
  "bindings": {
    "tab": "editor::SuperTab"
  },
}
```
