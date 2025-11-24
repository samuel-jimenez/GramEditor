#!/usr/bin/env sh
set -eu

# Downloads the latest tarball from https://zed.dev/releases and unpacks it
# into ~/.local/. If you'd prefer to do this manually, instructions are at
# https://tehanu.liten.app/docs/linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${TEHANU_CHANNEL:-stable}"
    # Use TMPDIR if available (for environments with non-standard temp directories)
    if [ -n "${TMPDIR:-}" ] && [ -d "${TMPDIR}" ]; then
        temp="$(mktemp -d "$TMPDIR/zed-XXXXXX")"
    else
        temp="$(mktemp -d "/tmp/zed-XXXXXX")"
    fi

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        macos-arm64* | linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        macos-x86* | linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"

    if [ "$(command -v zed)" = "$HOME/.local/bin/tehanu" ]; then
        echo "Tehanu has been installed. Run with 'zed'"
    else
        echo "To run Tehanu from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run Tehanu now, '~/.local/bin/tehanu'"
    fi
}

linux() {
    if [ -n "${TEHANU_BUNDLE_PATH:-}" ]; then
        cp "$TEHANU_BUNDLE_PATH" "$temp/tehanu-linux-$arch.tar.gz"
    else
        echo "Downloading Tehanu"
        curl "https://cloud.zed.dev/releases/$channel/latest/download?asset=zed&arch=$arch&os=linux&source=install.sh" > "$temp/tehanu-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="se.ziran.Tehanu"
        ;;
      nightly)
        appid="se.ziran.Tehanu-Nightly"
        ;;
      preview)
        appid="se.ziran.Tehanu-Preview"
        ;;
      dev)
        appid="se.ziran.Tehanu-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="se.ziran.Tehanu"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/tehanu$suffix.app"
    mkdir -p "$HOME/.local/tehanu$suffix.app"
    tar -xzf "$temp/tehanu-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/tehanu$suffix.app/bin/tehanu" ]; then
        ln -sf "$HOME/.local/tehanu$suffix.app/bin/tehanu" "$HOME/.local/bin/tehanu"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/tehanu$suffix.app/bin/cli" "$HOME/.local/bin/tehanu"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    cp "$HOME/.local/tehanu$suffix.app/share/applications/tehanu$suffix.desktop" "${desktop_file_path}"
    sed -i "s|Icon=tehanu|Icon=$HOME/.local/tehanu$suffix.app/share/icons/hicolor/512x512/apps/tehanu.png|g" "${desktop_file_path}"
    sed -i "s|Exec=zed|Exec=$HOME/.local/tehanu$suffix.app/bin/tehanu|g" "${desktop_file_path}"
}

macos() {
    echo "Downloading Tehanu"
    curl "https://cloud.zed.dev/releases/$channel/latest/download?asset=zed&os=macos&arch=$arch&source=install.sh" > "$temp/Tehanu-$arch.dmg"
    hdiutil attach -quiet "$temp/Tehanu-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [ -d "/Applications/$app" ]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    mkdir -p "$HOME/.local/bin"
    # Link the binary
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/tehanu"
}

main "$@"
