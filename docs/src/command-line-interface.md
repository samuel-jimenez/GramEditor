# Command-line Interface

Tehanu has a CLI, on Linux this should come with the distribution's Tehanu package (binary name can vary from distribution to distribution, `zed` will be used later for brevity).
For macOS, the CLI comes in the same package with the editor binary, and could be installed into the system with the `cli: install` Tehanu command which will create a symlink to the `/usr/local/bin/zed`.
It can also be built from source out of the `cli` crate in this repository.

Use `zed --help` to see the full list of capabilities.
General highlights:

- Opening another empty Tehanu window: `zed`

- Opening a file or directory in Tehanu: `zed /path/to/entry` (use `-n` to open in the new window)

- Reading from stdin: `ps axf | zed -`

- Starting Tehanu with logs in the terminal: `zed --foreground`

- Uninstalling Tehanu and all its related files: `zed --uninstall`
