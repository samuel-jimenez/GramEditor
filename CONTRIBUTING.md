# Contributing to Ludd

The goal of Ludd is to take the AI-infested, bloated whale carcass that is the
Zed code editor and strip it down into a decent, modern editor without any of
the shady bits.

If this sounds interesting to you, feel free to help out.

It should go without saying but AI is strictly banned from this project.

## Bird's-eye view of Ludd

We suggest you keep the [Ludd glossary](docs/src/development/glossary.md) at your side when starting out. It lists and explains some of the structures and terms you will see throughout the codebase.

Ludd is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`gpui`](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Ludd.
- [`editor`](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Ludd. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [`project`](/crates/project) manages files and navigation within the filetree. It is also Ludd's side of communication with LSP.
- [`workspace`](/crates/workspace) handles local state serialization and groups projects together.
- [`vim`](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [`lsp`](/crates/lsp) handles communication with external LSP server.
- [`language`](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [`theme`](/crates/theme) defines the theme system and provides a default theme.
- [`ui`](/crates/ui) is a collection of UI components and common patterns used throughout Ludd.
- [`cli`](/crates/cli) is the CLI crate which invokes the Ludd binary.
- [`zed`](/crates/zed) is where all things come together, and the `main` entry point for Ludd.

