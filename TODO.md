# TODO

- Find a solution for the remote_server situation...

  The problem is that in dev mode we can just build the remote
  server executable and that works fine, but in stable mode the
  only way to get it is by downloading from a server (that's what
  zed does) either directly on the remote or on the client machine.
  Well, I don't want to do that...

  - Build a minimal zig-based remote_server executable for all targets and bundle?

- Replace the extension model

  - Figure out what the extension API basically looks like and
  reimplement in Lua. Extensions are now lua scripts, we can bundle
  a lua bundle with extensions in the installer.

- Reintroduce collab editing using https://github.com/teamtype/teamtype

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

