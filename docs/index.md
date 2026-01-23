# Documentation

Gram is a **hard fork** of the Zed editor, with
the following list (incomplete) of changes:

- Integrated documentation viewer
- No license agreement on installation
- All AI integration has been removed
- All Telemetry has been removed
- No proprietary server component
- No auto updates
- All collaboration integration has been removed
- Extensions are installed from source only
- Partial support for WASM extensions (due to AI removal)

## Getting Started

- [The Mission](./mission.md)
- [Getting Started](./getting-started.md)
- [Uninstall](./uninstall.md)
- [Troubleshooting](./troubleshooting.md)

### Migrating

- From [Zed](./migrate/zed.md)
- From [VS Code](./migrate/vs-code.md)

## Features

- [Debugger](./debugger.md): Integrated support for DAP, the debugger adapter
  protocol.
- [Remote Development](./remote-development.md): Connect to remote servers via
  SSH and edit as if working on a local project.
- [Extensions](./extensions.md): Add support for additional languages, themes
  and icons using the extension system.

## Development

- [Development](./development.md)
  - [macOS](./development/macos.md)
  - [Linux](./development/linux.md)
  - [Windows](./development/windows.md)
  - [FreeBSD](./development/freebsd.md)
  - [Using Debuggers](./development/debuggers.md)
  - [Glossary](./development/glossary.md)
- [Debugging Crashes](./development/debugging-crashes.md)

## Configuration

- [Configuring Gram](./configuring-gram.md)
- [Configuring Languages](./configuring-languages.md)
  - [Toolchains](./toolchains.md)
- [Key bindings](./key-bindings.md)
  - [All Actions](./all-actions.md)
- [Snippets](./snippets.md)
- [Themes](./themes.md)
- [Icon Themes](./icon-themes.md)
- [Visual Customization](./visual-customization.md)
- [Vim Mode](./vim.md)
- [Helix Mode](./helix.md)

## Using Gram

- [Multibuffers](./multibuffers.md)
- [Command Palette](./command-palette.md)
- [Command-line Interface](./command-line-interface.md)
- [Outline Panel](./outline-panel.md)
- [Code Completions](./completions.md)
- [Git](./git.md)
- [Debugger](./debugger.md)
- [Diagnostics](./diagnostics.md)
- [Tasks](./tasks.md)
- [Tab Switcher](./tab-switcher.md)
- [Remote Development](./remote-development.md)
- [Environment Variables](./environment.md)
- [REPL](./repl.md)

## Platform Support

- [Windows](./windows.md)
- [Linux](./linux.md)

## Legal note on accepting contributions

If you have previously installed Zed and agreed to their license agreement, you
may be legally prevented from contributing to Gram despite the open source
license of the code. I am not a lawyer and I suspect that the license that they
use would not hold up at least in European court, but I don't know. For that
exact reason, I never agreed to their license. This is the main reason this fork
even exists.

If you do want to contribute patches, you will have to accept full responsibility
for ensuring and warranting that you are legally allowed to do so.

## You are the community

Gram is proudly open source, in spirit, not just in words. That said, we have
strong opinions about what we want to include in the editor. For example, the
main reason for this fork from Zed is to remove certain "features" that we
disagree with, morally. However, you are of course free to make it your own in
any way you see fit.

I don't use AI and if you submit a patch created with AI I might reject it
(especially if it doesn't work and you can't explain or fix it). That said, the
upstream Zed project does use it, so this project is not AI free in any sense.

There is no official discord or reddit community.

## Extensions

> The Zed extension system relies on a closed-source server component, which is
> stripped from Gram. Instead, all extensions have to be built from source.
> Currently, there is no extension registry so the extensions have to be
> installed either via the suggestion popups or an URL and WASM extensions need
> rustup installed in order to compile.

- [Overview](./extensions.md)
- [Installing Extensions](./extensions/installing-extensions.md)
- [Developing Extensions](./extensions/developing-extensions.md)
- [Extension Capabilities](./extensions/capabilities.md)
- [Language Extensions](./extensions/languages.md)
- [Debugger Extensions](./extensions/debugger-extensions.md)
- [Theme Extensions](./extensions/themes.md)
- [Icon Theme Extensions](./extensions/icon-themes.md)

## Language Support

- [All Languages](./languages.md)
- [Ansible](./languages/ansible.md)
- [AsciiDoc](./languages/asciidoc.md)
- [Astro](./languages/astro.md)
- [Bash](./languages/bash.md)
- [Biome](./languages/biome.md)
- [C](./languages/c.md)
- [C++](./languages/cpp.md)
- [C#](./languages/csharp.md)
- [Clojure](./languages/clojure.md)
- [CSS](./languages/css.md)
- [Dart](./languages/dart.md)
- [Deno](./languages/deno.md)
- [Diff](./languages/diff.md)
- [Docker](./languages/docker.md)
- [Elixir](./languages/elixir.md)
- [Elm](./languages/elm.md)
- [Emmet](./languages/emmet.md)
- [Erlang](./languages/erlang.md)
- [Fish](./languages/fish.md)
- [GDScript](./languages/gdscript.md)
- [Gleam](./languages/gleam.md)
- [GLSL](./languages/glsl.md)
- [Go](./languages/go.md)
- [Groovy](./languages/groovy.md)
- [Haskell](./languages/haskell.md)
- [Helm](./languages/helm.md)
- [HTML](./languages/html.md)
- [Java](./languages/java.md)
- [JavaScript](./languages/javascript.md)
- [Julia](./languages/julia.md)
- [JSON](./languages/json.md)
- [Jsonnet](./languages/jsonnet.md)
- [Kotlin](./languages/kotlin.md)
- [Lua](./languages/lua.md)
- [Luau](./languages/luau.md)
- [Makefile](./languages/makefile.md)
- [Markdown](./languages/markdown.md)
- [Nim](./languages/nim.md)
- [OCaml](./languages/ocaml.md)
- [OpenTofu](./languages/opentofu.md)
- [PHP](./languages/php.md)
- [PowerShell](./languages/powershell.md)
- [Prisma](./languages/prisma.md)
- [Proto](./languages/proto.md)
- [PureScript](./languages/purescript.md)
- [Python](./languages/python.md)
- [R](./languages/r.md)
- [Rego](./languages/rego.md)
- [ReStructuredText](./languages/rst.md)
- [Racket](./languages/racket.md)
- [Roc](./languages/roc.md)
- [Ruby](./languages/ruby.md)
- [Rust](./languages/rust.md)
- [Scala](./languages/scala.md)
- [Scheme](./languages/scheme.md)
- [Shell Script](./languages/sh.md)
- [SQL](./languages/sql.md)
- [Svelte](./languages/svelte.md)
- [Swift](./languages/swift.md)
- [Tailwind CSS](./languages/tailwindcss.md)
- [Terraform](./languages/terraform.md)
- [TOML](./languages/toml.md)
- [TypeScript](./languages/typescript.md)
- [Uiua](./languages/uiua.md)
- [Vue](./languages/vue.md)
- [XML](./languages/xml.md)
- [YAML](./languages/yaml.md)
- [Yara](./languages/yara.md)
- [Yarn](./languages/yarn.md)
- [Zig](./languages/zig.md)
