# Tehanu on Linux

## Standard Installation

For most people we recommend using the script on the [download](https://tehanu.liten.app/download) page to install Tehanu:

```sh
curl -f https://tehanu.liten.app/install.sh | sh
```

We also offer a preview build of Tehanu which receives updates about a week ahead of stable. You can install it with:

```sh
curl -f https://tehanu.liten.app/install.sh | TEHANU_CHANNEL=preview sh
```

The Tehanu installed by the script works best on systems that:

- have a Vulkan compatible GPU available (for example Linux on an M-series macBook)
- have a system-wide glibc (NixOS and Alpine do not by default)
  - x86_64 (Intel/AMD): glibc version >= 2.31 (Ubuntu 20 and newer)
  - aarch64 (ARM): glibc version >= 2.35 (Ubuntu 22 and newer)

Both Nix and Alpine have third-party Tehanu packages available (though they are currently a few weeks out of date). If you'd like to use our builds they do work if you install a glibc compatibility layer. On NixOS you can try [nix-ld](https://github.com/Mic92/nix-ld), and on Alpine [gcompat](https://wiki.alpinelinux.org/wiki/Running_glibc_programs).

You will need to build from source for:

- architectures other than 64-bit Intel or 64-bit ARM (for example a 32-bit or RISC-V machine)
- Redhat Enterprise Linux 8.x, Rocky Linux 8, AlmaLinux 8, Amazon Linux 2 on all architectures
- Redhat Enterprise Linux 9.x, Rocky Linux 9.3, AlmaLinux 8, Amazon Linux 2023 on aarch64 (x86_x64 OK)

## Other ways to install Tehanu on Linux

Tehanu is open source, and [you can install from source](./development/linux.md).

### Installing via a package manager

There are several third-party Tehanu packages for various Linux distributions and package managers, sometimes under `tehanu-editor`. You may be able to install Tehanu using these packages:

- Flathub: [`se.ziran.Tehanu`](https://flathub.org/apps/se.ziran.Tehanu)
- Arch: [`tehanu`](https://archlinux.org/packages/extra/x86_64/tehanu/)
- Arch (AUR): [`tehanu-git`](https://aur.archlinux.org/packages/tehanu-git), [`tehanu-preview`](https://aur.archlinux.org/packages/tehanu-preview), [`tehanu-preview-bin`](https://aur.archlinux.org/packages/tehanu-preview-bin)
- Alpine: `tehanu` ([aarch64](https://pkgs.alpinelinux.org/package/edge/testing/aarch64/tehanu)) ([x86_64](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/tehanu))
- Nix: `tehanu-editor` ([unstable](https://search.nixos.org/packages?channel=unstable&show=tehanu-editor))
- Fedora/Ultramarine (Terra): [`tehanu`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/tehanu/stable), [`tehanu-preview`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/tehanu/preview), [`tehanu-nightly`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/tehanu/nightly)
- Solus: [`tehanu`](https://github.com/getsolus/packages/tree/main/packages/z/tehanu)
- Parabola: [`tehanu`](https://www.parabola.nu/packages/extra/x86_64/tehanu/)
- Manjaro: [`tehanu`](https://packages.manjaro.org/?query=tehanu)
- ALT Linux (Sisyphus): [`tehanu`](https://packages.altlinux.org/en/sisyphus/srpms/tehanu/)
- AOSC OS: [`tehanu`](https://packages.aosc.io/packages/tehanu)

See [Repology](https://repology.org/project/tehanu-editor/versions) for a list of Tehanu packages in various repositories.

### Community

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Tehanu we package (a common change is to rename the binary to `tehanuit` or `tehanuitor` to avoid conflicting with other packages).

We'd love your help making Tehanu available for everyone. If Tehanu is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-tehanu).

The packages in this section provide binary installs for Tehanu but are not official packages within the associated distributions. These packages are maintained by community members and as such a higher level of caution should be taken when installing them.

#### Debian

Tehanu is available in [this community-maintained repository](https://debian.griffo.io/).

Instructions for each version are available in the README of the repository where packages are built.
Build, packaging and instructions for each version are available in the README of the [repository](https://github.com/dariogriffo/tehanu-debian)

### Downloading manually

If you'd prefer, you can install Tehanu by downloading our pre-built .tar.gz. This is the same artifact that our install script uses, but you can customize the location of your installation by modifying the instructions below:

Download the `.tar.gz` file:

- [tehanu-linux-x86_64.tar.gz](https://cloud.tehanu.liten.app/releases/stable/latest/download?asset=tehanu&arch=x86_64&os=linux&source=docs)
  ([preview](https://cloud.tehanu.liten.app/releases/preview/latest/download?asset=tehanu&arch=x86_64&os=linux&source=docs))
- [tehanu-linux-aarch64.tar.gz](https://cloud.tehanu.liten.app/releases/stable/latest/download?asset=tehanu&arch=aarch64&os=linux&source=docs)
  ([preview](https://cloud.tehanu.liten.app/releases/preview/latest/download?asset=tehanu&arch=aarch64&os=linux&source=docs))

Then ensure that the `tehanu` binary in the tarball is on your path. The easiest way is to unpack the tarball and create a symlink:

```sh
mkdir -p ~/.local
# extract tehanu to ~/.local/tehanu.app/
tar -xvf <path/to/download>.tar.gz -C ~/.local
# link the tehanu binary to ~/.local/bin (or another directory in your $PATH)
ln -sf ~/.local/tehanu.app/bin/tehanu ~/.local/bin/tehanu
```

If you'd like integration with an XDG-compatible desktop environment, you will also need to install the `.desktop` file:

```sh
cp ~/.local/tehanu.app/share/applications/tehanu.desktop ~/.local/share/applications/se.ziran.Tehanu.desktop
sed -i "s|Icon=tehanu|Icon=$HOME/.local/tehanu.app/share/icons/hicolor/512x512/apps/tehanu.png|g" ~/.local/share/applications/se.ziran.Tehanu.desktop
sed -i "s|Exec=tehanu|Exec=$HOME/.local/tehanu.app/libexec/tehanu-editor|g" ~/.local/share/applications/se.ziran.Tehanu.desktop
```

## Uninstalling Tehanu

### Standard Uninstall

If Tehanu was installed using the default installation script, it can be uninstalled by supplying the `--uninstall` flag to the `tehanu` shell command

```sh
tehanu --uninstall
```

If there are no errors, the shell will then prompt you whether you'd like to keep your preferences or delete them. After making a choice, you should see a message that Tehanu was successfully uninstalled.

In the case that the `tehanu` shell command was not found in your PATH, you can try one of the following commands

```sh
$HOME/.local/bin/tehanu --uninstall
```

or

```sh
$HOME/.local/tehanu.app/bin.tehanu --uninstall
```

The first case might fail if a symlink was not properly established between `$HOME/.local/bin/tehanu` and `$HOME/.local/tehanu.app/bin.tehanu`. But the second case should work as long as Tehanu was installed to its default location.

If Tehanu was installed to a different location, you must invoke the `tehanu` binary stored in that installation directory and pass the `--uninstall` flag to it in the same format as the previous commands.

### Package Manager

If Tehanu was installed using a package manager, please consult the documentation for that package manager on how to uninstall a package.

## Troubleshooting

Linux works on a large variety of systems configured in many different ways. We primarily test Tehanu on a vanilla Ubuntu setup, as it is the most common distribution our users use, that said we do expect it to work on a wide variety of machines.

### Tehanu fails to start

If you see an error like "/lib64/libc.so.6: version 'GLIBC_2.29' not found" it means that your distribution's version of glibc is too old. You can either upgrade your system, or [install Tehanu from source](./development/linux.md).

### Graphics issues

#### Tehanu fails to open windows

Tehanu requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Tehanu fails to load, it is possible that Vulkan is the culprit.

If you see a notification saying `Tehanu failed to open a window: NoSupportedDeviceFound` this means that Vulkan cannot find a compatible GPU. you can try running [vkcube](https://github.com/krh/vkcube) (usually available as part of the `vulkaninfo` or `vulkan-tools` package on various distributions) to try to troubleshoot where the issue is coming from like so:

```
vkcube
```

> **_Note_**: Try running in both X11 and wayland modes by running `vkcube -m [x11|wayland]`. Some versions of `vkcube` use `vkcube` to run in X11 and `vkcube-wayland` to run in wayland.

This should output a line describing your current graphics setup and show a rotating cube. If this does not work, you should be able to fix it by installing Vulkan compatible GPU drivers, however in some cases there is no Vulkan support yet.

You can find out which graphics card Tehanu is using by looking in the Tehanu log (`~/.local/share/tehanu/logs/Tehanu.log`) for `Using GPU: ...`.

If you see errors like `ERROR_INITIALIZATION_FAILED` or `GPU Crashed` or `ERROR_SURFACE_LOST_KHR` then you may be able to work around this by installing different drivers for your GPU, or by selecting a different GPU to run on. (See [#14225](https://github.com/tehanu-industries/tehanu/issues/14225))

On some systems the file `/etc/prime-discrete` can be used to enforce the use of a discrete GPU using [PRIME](https://wiki.archlinux.org/title/PRIME). Depending on the details of your setup, you may need to change the contents of this file to "on" (to force discrete graphics) or "off" (to force integrated graphics).

On others, you may be able to the environment variable `DRI_PRIME=1` when running Tehanu to force the use of the discrete GPU.

If you're using an AMD GPU and Tehanu crashes when selecting long lines, try setting the `TEHANU_PATH_SAMPLE_COUNT=0` environment variable. (See [#26143](https://github.com/tehanu-industries/tehanu/issues/26143))

If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See [#13880](https://github.com/tehanu-industries/tehanu/issues/13880))

If you are using `amdvlk`, the default open-source AMD graphics driver, you may find that Tehanu consistently fails to launch. This is a known issue for some users, for example on Omarchy (see issue [#28851](https://github.com/tehanu-industries/tehanu/issues/28851)). To fix this, you will need to use a different driver. We recommend removing the `amdvlk` and `lib32-amdvlk` packages and installing `vulkan-radeon` instead (see issue [#14141](https://github.com/tehanu-industries/tehanu/issues/14141)).

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps that translate well to most distributions.

#### Forcing Tehanu to use a specific GPU

There are a few different ways to force Tehanu to use a specific GPU:

##### Option A

You can use the `TEHANU_DEVICE_ID={device_id}` environment variable to specify the device ID of the GPU you wish to have Tehanu use.

You can obtain the device ID of your GPU by running `lspci -nn | grep VGA` which will output each GPU on one line like:

```
08:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA104 [GeForce RTX 3070] [10de:2484] (rev a1)
```

where the device ID here is `2484`. This value is in hexadecimal, so to force Tehanu to use this specific GPU you would set the environment variable like so:

```
TEHANU_DEVICE_ID=0x2484 tehanu
```

Make sure to export the variable if you choose to define it globally in a `.bashrc` or similar.

##### Option B

If you are using Mesa, you can run `MESA_VK_DEVICE_SELECT=list tehanu --foreground` to get a list of available GPUs and then export `MESA_VK_DEVICE_SELECT=xxxx:yyyy` to choose a specific device. Furthermore, you can fallback to xwayland with an additional export of `WAYLAND_DISPLAY=""`.

##### Option C

Using [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser).

#### Reporting graphics issues

If Vulkan is configured correctly, and Tehanu is still not working for you, please [file an issue](https://github.com/tehanu-industries/tehanu) with as much information as possible.

When reporting issues where Tehanu fails to start due to graphics initialization errors on GitHub, it can be impossible to run the `tehanu: copy system specs into clipboard` command like we instruct you to in our issue template. We provide an alternative way to collect the system specs specifically for this situation.

Passing the `--system-specs` flag to Tehanu like

```sh
tehanu --system-specs
```

will print the system specs to the terminal like so. It is strongly recommended to copy the output verbatim into the issue on GitHub, as it uses markdown formatting to ensure the output is readable.

Additionally, it is extremely beneficial to provide the contents of your Tehanu log when reporting such issues. The log is usually located at `~/.local/share/tehanu/logs/Tehanu.log`. The recommended process for producing a helpful log file is as follows:

```sh
truncate -s 0 ~/.local/share/tehanu/logs/Tehanu.log # Clear the log file
TEHANU_LOG=blade_graphics=info tehanu .
cat ~/.local/share/tehanu/logs/Tehanu.log
# copy the output
```

Or, if you have the Tehanu cli setup, you can do

```sh
TEHANU_LOG=blade_graphics=info /path/to/tehanu/cli --foreground .
# copy the output
```

It is also highly recommended when pasting the log into a github issue, to do so with the following template:

> **_Note_**: The whitespace in the template is important, and will cause incorrect formatting if not preserved.

````
<details><summary>Tehanu Log</summary>

```
{tehanu log contents}
```

</details>
````

This will cause the logs to be collapsed by default, making it easier to read the issue.

### I can't open any files

### Clicking links isn't working

These features are provided by XDG desktop portals, specifically:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`

Some window managers, such as `Hyprland`, don't provide a file picker by default. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point for alternatives.

### Tehanu isn't remembering my API keys

### Tehanu isn't remembering my login

These feature also requires XDG desktop portals, specifically:

- `org.freedesktop.portal.Secret` or
- `org.freedesktop.Secrets`

Tehanu needs a place to securely store secrets such as your Tehanu login cookie or your OpenAI API Keys and we use a system provided keychain to do this. Examples of packages that provide this are `gnome-keyring`, `KWallet` and `keepassxc` among others.

### Could not start inotify

Tehanu relies on inotify to watch your filesystem for changes. If you cannot start inotify then Tehanu will not work reliably.

If you are seeing "too many open files" then first try `sysctl fs.inotify`.

- You should see that max_user_instances is 128 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_instances=1024`). Tehanu needs only 1 inotify instance.
- You should see that `max_user_watches` is 8000 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_watches=64000`). Tehanu needs one watch per directory in all your open projects + one per git repository + a handful more for settings, themes, keymaps, extensions.

It is also possible that you are running out of file descriptors. You can check the limits with `ulimit` and update them by editing `/etc/security/limits.conf`.

### No sound or wrong output device

If you're not hearing any sound in Tehanu or the audio is routed to the wrong device, it could be due to a mismatch between audio systems. Tehanu relies on ALSA, while your system may be using PipeWire or PulseAudio. To resolve this, you need to configure ALSA to route audio through PipeWire/PulseAudio.

If your system uses PipeWire:

1. **Install the PipeWire ALSA plugin**

   On Debian-based systems, run:

   ```bash
   sudo apt install pipewire-alsa
   ```

2. **Configure ALSA to use PipeWire**

   Add the following configuration to your ALSA settings file. You can use either `~/.asoundrc` (user-level) or `/etc/asound.conf` (system-wide):

   ```bash
   pcm.!default {
       type pipewire
   }

   ctl.!default {
       type pipewire
   }
   ```

3. **Restart your system**

### Forcing X11 scale factor

On X11 systems, Tehanu automatically detects the appropriate scale factor for high-DPI displays. The scale factor is determined using the following priority order:

1. `GPUI_X11_SCALE_FACTOR` environment variable (if set)
2. `Xft.dpi` from X resources database (xrdb)
3. Automatic detection via RandR based on monitor resolution and physical size

If you want to customize the scale factor beyond what Tehanu detects automatically, you have several options:

#### Check your current scale factor

You can verify if you have `Xft.dpi` set:

```sh
xrdb -query | grep Xft.dpi
```

If this command returns no output, Tehanu is using RandR (X11's monitor management extension) to automatically calculate the scale factor based on your monitor's reported resolution and physical dimensions.

#### Option 1: Set Xft.dpi (X Resources Database)

`Xft.dpi` is a standard X11 setting that many applications use for consistent font and UI scaling. Setting this ensures Tehanu scales the same way as other X11 applications that respect this setting.

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

Restart Tehanu for the changes to take effect.

#### Option 2: Use the GPUI_X11_SCALE_FACTOR environment variable

This Tehanu-specific environment variable directly sets the scale factor, bypassing all automatic detection.

```sh
GPUI_X11_SCALE_FACTOR=1.5 tehanu
```

You can use decimal values (e.g., `1.25`, `1.5`, `2.0`) or set `GPUI_X11_SCALE_FACTOR=randr` to force RandR-based detection even when `Xft.dpi` is set.

To make this permanent, add it to your shell profile or desktop entry.

#### Option 3: Adjust system-wide RandR DPI

This changes the reported DPI for your entire X11 session, affecting how RandR calculates scaling for all applications that use it.

Add this to your `.xprofile` or `.xinitrc`:

```sh
xrandr --dpi 192
```

Replace `192` with your desired DPI value. This affects the system globally and will be used by Tehanu's automatic RandR detection when `Xft.dpi` is not set.

### Font rendering parameters

When using Blade rendering (Linux platforms and self-compiled builds with the Blade renderer enabled), Tehanu reads `TEHANU_FONTS_GAMMA` and `TEHANU_FONTS_GRAYSCALE_ENHANCED_CONTRAST` environment variables for the values to use for font rendering.

`TEHANU_FONTS_GAMMA` corresponds to [getgamma](https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwriterenderingparams-getgamma) values.
Allowed range [1.0, 2.2], other values are clipped.
Default: 1.8

`TEHANU_FONTS_GRAYSCALE_ENHANCED_CONTRAST` corresponds to [getgrayscaleenhancedcontrast](https://learn.microsoft.com/en-us/windows/win32/api/dwrite_1/nf-dwrite_1-idwriterenderingparams1-getgrayscaleenhancedcontrast) values.
Allowed range: [0.0, ..), other values are clipped.
Default: 1.0
