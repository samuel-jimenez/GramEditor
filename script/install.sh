#!/usr/bin/env bash
# shellcheck shell=bash
# Installation script for Linux
set -eu

err() {
  echo "$1" >&2
  exit 1
}

has_command() {
  if ! command -v $1; then
    err "Required command not found: $1"
  fi
}

usage() {
  echo "
Usage: ${0##*/} [options] [BUNDLE]
Install Gram on Linux from a tar bundle.

Options:
  -h, --help       Display this help and exit.
  --build          Build the tar bundle before installation.
  --prefix PREFIX Install into PREFIX (default ~/.local).
  "
}

GRAM_BUILD_TARBALL=no
GRAM_INSTALL_PREFIX="$HOME/.local"
GRAM_BUNDLE_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        --build)
            GRAM_BUILD_TARBALL=true
            shift
            ;;
        --prefix)
            shift
            [[ $# -lt 1 ]] && err "Expected PREFIX"
            GRAM_INSTALL_PREFIX="$1"
            shift
            ;;
        --)
            shift
            break
            ;;
        -*)
            echo "Unknown option: $1" >&2
            help_info
            exit 1
            ;;
        *)
            if [[ $# -gt 1 ]]; then
              err "Too many arguments, expected [BUNDLE]"
            fi
            if [[ $# -eq 1 ]]; then
              GRAM_BUNDLE_FILE="$1"
              shift
            fi
            ;;
    esac
done

version="$(script/get-crate-version gram)"
host_line="$(rustc --version --verbose | grep "host")"
target_triple=${host_line#*: }
arch="$(echo $target_triple | awk -F - '{print $1}')"

if [[ "$GRAM_BUILD_TARBALL" = "true" ]]; then
  echo "dev" > ./crates/gram/RELEASE_CHANNEL
  ./script/bundle-linux
  GRAM_BUNDLE_FILE="target/release/gram-linux-$arch.tar.gz"
elif [ "$GRAM_BUNDLE_FILE" = "" ]; then
  GRAM_BUNDLE_FILE="gram-linux-$arch-$version.tar.gz"
fi
[[ ! -f "$GRAM_BUNDLE_FILE" ]] && err "$GRAM_BUNDLE_FILE not found, exiting..."

channel=stable
if tar ztf "$GRAM_BUNDLE_FILE" | head -1 | grep -q "dev"; then
  channel=dev
fi
suffix=""
if [ "$channel" != "stable" ]; then
  suffix="-$channel"
fi
mkdir -p "$GRAM_INSTALL_PREFIX/gram$suffix.app"
mkdir -p "$GRAM_INSTALL_PREFIX/bin" "$GRAM_INSTALL_PREFIX/share/applications"
tar -xzf "$GRAM_BUNDLE_FILE" -C "$GRAM_INSTALL_PREFIX/"

ln -sf "$GRAM_INSTALL_PREFIX/gram$suffix.app/bin/gram" "$HOME/.local/bin/gram"

desktop_file_path="$GRAM_INSTALL_PREFIX/share/applications/gram${suffix}.desktop"
src_dir="$GRAM_INSTALL_PREFIX/gram$suffix.app/share/applications"
cp "$src_dir/gram${suffix}.desktop" "${desktop_file_path}"

sed -i "s|Icon=gram|Icon=$GRAM_INSTALL_PREFIX/gram$suffix.app/share/icons/hicolor/512x512/apps/gram.png|g" "${desktop_file_path}"
sed -i "s|Exec=gram|Exec=$GRAM_INSTALL_PREFIX/gram$suffix.app/bin/gram|g" "${desktop_file_path}"

echo "Installation to $GRAM_INSTALL_PREFIX complete."

