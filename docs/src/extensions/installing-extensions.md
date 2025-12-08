# Installing Extensions

You can search for extensions by launching the Gram Extension Gallery by pressing {#kb gram::Extensions} , opening the command palette and selecting {#action gram::Extensions} or by selecting "Gram > Extensions" from the menu bar.

Here you can view the extensions that you currently have installed or search and install new ones.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Gram/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/zed/extensions` or `~/.local/share/zed/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

