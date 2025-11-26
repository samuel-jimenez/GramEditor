# TODO

- I broke the dev extension installation..

- Add back support for downloading extensions (but still building them locally)

- No extensions show up in the UI...

- Sort themes alphabetically in the theme picker

- Replace the icons with https://github.com/cotyhamilton/zed-emoji-icon-theme

- Sniff out all the AI-written text and get rid of it

- LSP queries don't work on remote connection:

  ```
  2025-11-24T11:21:47+01:00 ERROR [crates/project/src/lsp_store.rs:7448] sending LSP proto request

  Caused by:
      RPC request LspQuery failed: unknown buffer id 38654705741
  2025-11-24T11:21:47+01:00 ERROR [remote::remote_client] (remote server) server:error handling message. type:LspQuery, error:unknown buffer id 38654705741
  ```

  ```
  2025-11-24T11:20:58+01:00 ERROR [crates/editor/src/linked_editing_ranges.rs:101] RPC request LinkedEditingRange failed: unknown buffer id 38654705741
  2025-11-24T11:20:58+01:00 ERROR [crates/editor/src/editor.rs:6512] RPC request GetDocumentHighlights failed: unknown buffer id 38654705741
  ```

- eslint notifications for our project don't work:

  ```
  2025-11-24T11:00:58+01:00 INFO  [lsp] (remote server) Language server with id 2 sent unhandled notification eslint/status:
  {
    "uri": "file:///<redacted>",
    "state": 1,
    "validationTime": 36
  }
  ```

