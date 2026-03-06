# Installing Extensions

Some extensions, particularly extensions that add language support, need to
compile the extension into a Wasm bundle. To do this, Gram uses Rust together
with the `wasm32-wasip2` target to build the Wasm extension on your local
machine.

You can either install these dependencies yourself, or alternatively if you have
[installed Rust via rustup](https://www.rust-lang.org/tools/install), Gram will
add the `wasm32-wasip2` target for you.

If you have installed Rust via a package manager but don't have the
`wasm32-wasip2` target installed, installing the extension will not work.

> **Note:** This is a core difference between Zed and Gram. Zed will download a
> precompiled wasm bundle from the Zed website when installing an extension, to
> avoid having to build the extension locally.

You can search for extensions by launching the Gram Extension Gallery by
pressing {#kb gram::Extensions} , opening the command palette and selecting
{#action gram::Extensions} or by selecting "Gram > Extensions" from the menu
bar.

Here you can view the extensions that you currently have installed or search and
install new ones.

You can also install extensions for your local file system, or via a git URL.

## Installing Local Extensions

From the extensions page, click the `Install Local` button (or the {#action gram::InstallExtensionFromFolder} action) and select the directory containing your extension. You should see the extension begin to install in the status bar.

## Installing Extensions From A Git URL

From the extensions page, click the `Install From URL` button (or the {#action gram::InstallExtensionFromUrl} action) and enter the git url for the extension. You should see the extension begin to install in the status bar.

> **Note:** Gram must be able to authenticate the git URL, but does not support SSH authentication. Using a public URL such as an `http(s)` git URL is recommended. Check the Gram logs to debug ({#action gram::OpenLog}).

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Gram/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/gram/extensions` or `~/.local/share/gram/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

## Removing extensions

Removing extensions are not yet supported via the UI. You will need to manually
remove the installed extension from the installation location on your local machine.
