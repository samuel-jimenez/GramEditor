# Gram on Linux

## Standard Installation

Gram is open source, and [you can install from source](./development/linux.md).

If there is a tarball available for your architecture at the [Gram Codeberg](https://codeberg.org/GramEditor/gram/releases) repository,
you can follow these instructions:


1. Download the [install.sh](https://codeberg.org/GramEditor/gram/raw/branch/main/script/install.sh) script.
2. Run the script.

   ```sh
   ./install.sh
   ```

   This will download latest release of Gram and install Gram to `$HOME/.local`.
   To install system-wide, use the `--prefix PREFIX` argument:

   ```sh
   ./install.sh --prefix /usr/local ./gram-linux-x86_64-1.0.0.tar.gz
   ```

## Troubleshooting

### Graphics issues

#### Gram fails to open windows

Gram requires a GPU to run effectively. Under the hood, it uses [Vulkan](https://www.vulkan.org/) to communicate with the GPU. If you are seeing problems with performance or Gram fails to load, it is possible that Vulkan is the culprit.

If you see a notification saying `Gram failed to open a window: NoSupportedDeviceFound` this means that Vulkan cannot find a compatible GPU. Try running [vkcube](https://github.com/krh/vkcube) (usually available as part of the `vulkaninfo` or `vulkan-tools` package on various distributions) to troubleshoot where the issue is coming from like so:

```
vkcube
```

> **_Note_**: Try running in both X11 and wayland modes by running `vkcube -m [x11|wayland]`. Some versions of `vkcube` use `vkcube` to run in X11 and `vkcube-wayland` to run in wayland.

This should output a line describing your current graphics setup and show a rotating cube. If this does not work, you should be able to fix it by installing Vulkan compatible GPU drivers, however in some cases there is no Vulkan support yet.

You can find out which graphics card Gram is using by looking in the Gram log (`~/.local/share/gram/logs/Gram.log`) for `Using GPU: ...`.

If you see errors like `ERROR_INITIALIZATION_FAILED` or `GPU Crashed` or `ERROR_SURFACE_LOST_KHR` then you may be able to work around this by installing different drivers for your GPU, or by selecting a different GPU to run on. (See [#14225](https://github.com/gram-industries/gram/issues/14225))

On some systems the file `/etc/prime-discrete` can be used to enforce the use of a discrete GPU using [PRIME](https://wiki.archlinux.org/title/PRIME). Depending on the details of your setup, you may need to change the contents of this file to "on" (to force discrete graphics) or "off" (to force integrated graphics).

On others, you may be able to the environment variable `DRI_PRIME=1` when running Gram to force the use of the discrete GPU.

If you're using an AMD GPU and Gram crashes when selecting long lines, try setting the `GRAM_PATH_SAMPLE_COUNT=0` environment variable. (See [#26143](https://github.com/gram-industries/gram/issues/26143))

If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See [#13880](https://github.com/gram-industries/gram/issues/13880))

If you are using `amdvlk`, the default open-source AMD graphics driver, you may find that Gram consistently fails to launch. This is a known issue for some users, for example on Omarchy (see issue [#28851](https://github.com/gram-industries/gram/issues/28851)). To fix this, you will need to use a different driver. We recommend removing the `amdvlk` and `lib32-amdvlk` packages and installing `vulkan-radeon` instead (see issue [#14141](https://github.com/gram-industries/gram/issues/14141)).

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps that translate well to most distributions.

#### Forcing Gram to use a specific GPU

There are a few different ways to force Gram to use a specific GPU:

##### Option A

You can use the `GRAM_DEVICE_ID={device_id}` environment variable to specify the device ID of the GPU you wish to have Gram use.

You can obtain the device ID of your GPU by running `lspci -nn | grep VGA` which will output each GPU on one line like:

```
08:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA104 [GeForce RTX 3070] [10de:2484] (rev a1)
```

where the device ID here is `2484`. This value is in hexadecimal, so to force Gram to use this specific GPU you would set the environment variable like so:

```
GRAM_DEVICE_ID=0x2484 gram
```

Make sure to export the variable if you choose to define it globally in a `.bashrc` or similar.

##### Option B

If you are using Mesa, you can run `MESA_VK_DEVICE_SELECT=list gram --foreground` to get a list of available GPUs and then export `MESA_VK_DEVICE_SELECT=xxxx:yyyy` to choose a specific device. Furthermore, you can fallback to xwayland with an additional export of `WAYLAND_DISPLAY=""`.

##### Option C

Using [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser).

#### Generating debug reports

Passing the `--system-specs` flag to Gram like

```sh
gram --system-specs
```

will print the system specs to the terminal.

The editor log is usually located at `~/.local/share/gram/logs/Gram.log`.

To generate a clean log file for debugging graphics issues, run:

```sh
truncate -s 0 ~/.local/share/gram/logs/Gram.log # Clear the log file
GRAM_LOG=blade_graphics=info gram .
cat ~/.local/share/gram/logs/Gram.log
# copy the output
```

Or, if you have the Gram cli setup, you can do

```sh
GRAM_LOG=blade_graphics=info /path/to/gram/cli --foreground .
# copy the output
```

### I can't open any files

### Clicking links isn't working

These features are provided by XDG desktop portals, specifically:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`

Some window managers, such as `Hyprland`, don't provide a file picker by default. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point for alternatives.

### Gram isn't remembering my API keys

### Gram isn't remembering my login

These feature also requires XDG desktop portals, specifically:

- `org.freedesktop.portal.Secret` or
- `org.freedesktop.Secrets`

Gram needs a place to securely store secrets such as your Gram login cookie or your OpenAI API Keys and we use a system provided keychain to do this. Examples of packages that provide this are `gnome-keyring`, `KWallet` and `keepassxc` among others.

### Could not start inotify

Gram relies on inotify to watch your filesystem for changes. If you cannot start inotify then Gram will not work reliably.

If you are seeing "too many open files" then first try `sysctl fs.inotify`.

- You should see that max_user_instances is 128 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_instances=1024`). Gram needs only 1 inotify instance.
- You should see that `max_user_watches` is 8000 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_watches=64000`). Gram needs one watch per directory in all your open projects + one per git repository + a handful more for settings, themes, keymaps, extensions.

It is also possible that you are running out of file descriptors. You can check the limits with `ulimit` and update them by editing `/etc/security/limits.conf`.

### Forcing X11 scale factor

On X11 systems, Gram automatically detects the appropriate scale factor for high-DPI displays. The scale factor is determined using the following priority order:

1. `GPUI_X11_SCALE_FACTOR` environment variable (if set)
2. `Xft.dpi` from X resources database (xrdb)
3. Automatic detection via RandR based on monitor resolution and physical size

If you want to customize the scale factor beyond what Gram detects automatically, you have several options:

#### Check your current scale factor

You can verify if you have `Xft.dpi` set:

```sh
xrdb -query | grep Xft.dpi
```

If this command returns no output, Gram is using RandR (X11's monitor management extension) to automatically calculate the scale factor based on your monitor's reported resolution and physical dimensions.

#### Option 1: Set Xft.dpi (X Resources Database)

`Xft.dpi` is a standard X11 setting that many applications use for consistent font and UI scaling. Setting this ensures Gram scales the same way as other X11 applications that respect this setting.

Edit or create the `~/.Xresources` file:

```sh
vim ~/.Xresources
```

Add this line with your desired DPI:

```sh
Xft.dpi: 96
```

Common DPI values:

- `96` for standard 1x scaling
- `144` for 1.5x scaling
- `192` for 2x scaling
- `288` for 3x scaling

Load the configuration:

```sh
xrdb -merge ~/.Xresources
```

Restart Gram for the changes to take effect.

#### Option 2: Use the GPUI_X11_SCALE_FACTOR environment variable

This Gram-specific environment variable directly sets the scale factor, bypassing all automatic detection.

```sh
GPUI_X11_SCALE_FACTOR=1.5 gram
```

You can use decimal values (e.g., `1.25`, `1.5`, `2.0`) or set `GPUI_X11_SCALE_FACTOR=randr` to force RandR-based detection even when `Xft.dpi` is set.

To make this permanent, add it to your shell profile or desktop entry.

#### Option 3: Adjust system-wide RandR DPI

This changes the reported DPI for your entire X11 session, affecting how RandR calculates scaling for all applications that use it.

Add this to your `.xprofile` or `.xinitrc`:

```sh
xrandr --dpi 192
```

Replace `192` with your desired DPI value. This affects the system globally and will be used by Gram's automatic RandR detection when `Xft.dpi` is not set.

### Font rendering parameters

When using Blade rendering (Linux platforms and self-compiled builds with the Blade renderer enabled), Gram reads `GRAM_FONTS_GAMMA` and `GRAM_FONTS_GRAYSCALE_ENHANCED_CONTRAST` environment variables for the values to use for font rendering.

`GRAM_FONTS_GAMMA` corresponds to [getgamma](https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwriterenderingparams-getgamma) values.
Allowed range [1.0, 2.2], other values are clipped.
Default: 1.8

`GRAM_FONTS_GRAYSCALE_ENHANCED_CONTRAST` corresponds to [getgrayscaleenhancedcontrast](https://learn.microsoft.com/en-us/windows/win32/api/dwrite_1/nf-dwrite_1-idwriterenderingparams1-getgrayscaleenhancedcontrast) values.
Allowed range: [0.0, ..), other values are clipped.
Default: 1.0
