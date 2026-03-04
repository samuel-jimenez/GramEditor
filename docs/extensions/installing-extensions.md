# Installing Extensions

You can search for extensions by launching the Gram Extension Gallery by pressing {#kb gram::Extensions} , opening the command palette and selecting {#action gram::Extensions} or by selecting "Gram > Extensions" from the menu bar.

Here you can view the extensions that you currently have installed or search and install new ones.

You can also install extensions for your local file system, or via a git URL.

## Installing Local Extensions

From the extensions page, click the `Install Local` button (or the {#action gram::InstallExtensionFromFolder} action) and select the directory containing your extension. You should see the extension begin to install in the status bar.

## Installing Extensions From A Git URL

From the extensions page, click the `Install From URL` button (or the {#action gram::InstallExtensionFromUrl} action) and enter the git url for the extension. You should see the extension begin to install in the status bar.

**Note:** Gram must be able to authenticate the git URL but Gram does not support ssh authentication. Using a public URL such as an `http` git URL is recommended. Check the Gram logs to debug.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Gram/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/gram/extensions` or `~/.local/share/gram/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.
