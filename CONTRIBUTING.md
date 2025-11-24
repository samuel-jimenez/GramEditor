# Contributing to Galadriel

The over-arching goal for now is to excise the Zed codebase of all the shady
stuff so that only the good parts remain. First and foremost, that means
removing all AI integration and telemetry.

Unfortunately, this means that some genuinely useful functionality may be lost
in the process. Thus, this is the plan for this project:

- [x] Strip out the AI, the telemetry, the subscriptions, the auto installation.
- [ ] Make everything that broke in the process work again.
- [ ] Replace anything that was lost with better, open and respectful
  alternatives.

AI will never return. Some form of peer-to-peer collaboration support would
be cool to have, though.

Automatic installation of pre-built extensions is gone. Is there a way to make
the editor more extensible without losing that? Maybe an option is to add
something like the extensions planned for Helix.

Another option is to drop the extensions entirely and just build everything
into the editor: Support as many languages and themes as possible out of the
box.

If this sounds interesting to you, feel free to help out.

It should go without saying but AI is strictly banned from this project.

## Bird's-eye view of Galadriel

The [glossary](docs/src/development/glossary.md) lists and explains some of the structures and terms you will see throughout the codebase.

Galadriel is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`gpui`](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for the UI.
- [`editor`](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [`project`](/crates/project) manages files and navigation within the filetree. It is also the editor side of communication with LSP.
- [`workspace`](/crates/workspace) handles local state serialization and groups projects together.
- [`vim`](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [`lsp`](/crates/lsp) handles communication with external LSP server.
- [`language`](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [`theme`](/crates/theme) defines the theme system and provides a default theme.
- [`ui`](/crates/ui) is a collection of UI components and common patterns used throughout the editor.
- [`cli`](/crates/cli) is the CLI crate which invokes the el binary.
- [`el`](/crates/el) is where all things come together, and the `main` entry point for the project.

