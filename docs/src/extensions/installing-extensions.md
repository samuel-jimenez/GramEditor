# Installing Extensions

You can search for extensions by launching the Tehanu Extension Gallery by pressing {#kb tehanu::Extensions} , opening the command palette and selecting {#action tehanu::Extensions} or by selecting "Tehanu > Extensions" from the menu bar.

Here you can view the extensions that you currently have installed or search and install new ones.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Tehanu/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/zed/extensions` or `~/.local/share/zed/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

