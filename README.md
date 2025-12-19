# Gram

This project is a strongy opinionated fork of the Zed code editor. The main
rationale behind this fork is that I am curious about a new code editor suitable
for beginners, but I don't want anything to do with AI, I don't want my editor
to phone home to a US corporation, and I definitely don't want to sign in,
sign up or accept any terms of use to write code.

I have gotten to this point in my life without making the tools of my trade into
a subscription service from some of the worst companies ever to exist, and I
don't intend to start now.

One of my jobs recently has been as a teacher to new students learning to code
without any prior experience, and up until 2024 my editor of choice was VS
Code. It has been easy to install and set up, and comes with all the tooling
needed to work with Python which was the language of choice for the course.

However, as of 2025 VS Code has become completely unusable for beginners due to
the inclusion of very intrusive and disruptive AI tooling and the increasing
amount of malicious and harmful extensions being published. Imagine telling a
student to download and install VS Code and to write a basic "hello world"
program in Python. They will get as far as typing "pr-" at which point the
editor will start throwing nonsensical suggestions, prompts and distractions
at the student.

This is bad.

My hope is that Gram will be an editor that a student can download and install
and use out of the box with a friendly onboarding experience and configuration,
integrated help and documentation, debugging and language server features but
without pushing anything malicious, distracting or confusing.

## Manifesto

This project is first and foremost a source code editor. It aims to be a fast,
reliable and hackable tool for developers to use, reuse, share and modify. It
will _never_ contain, support or condone any of the following "features" that
_permeate_ the Gram code editor: AI, Telemetry, Proprietary server components,
third-party service integrations, Contributor Licenses, Terms of Service or
subscription fees.

We promise:

- NO AI
- NO TELEMETRY
- NO PROPRIETARY "COLLABORATION"
- NO CLA
- NO TERMS OF USE
- NO THIRD PARTY LICENSING AGREEMENTS
- NO SUBSCRIPTIONS
- NO AUTOMATIC INSTALLATION OR UPDATES

For more thoughts on this topic, see [MISSION.md](./MISSION.md).

## Installation

At the moment, you will need to build the editor from source. Eventually there
will be a website to download from.

Make sure you have Rust installed (via rustup, preferrably).

```sh
cargo run --release
```

## Developing

- [Building for macOS](./docs/src/development/macos.md)
- [Building for Linux](./docs/src/development/linux.md)
- [Building for Windows](./docs/src/development/windows.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to this
project.

## Licensing

The Gram editor is licensed under the GPLv3 or the AGPLv3 licenses. The Zed
editor codebase is triple-licensed and also allows use under the Apache 2
license, but any modifications made in _this_ code base are _NOT_ licensed under
Apache 2, only GPLv3 or AGPLv3.

This project is subject to the licenses of its original sources and
dependencies.

License information for third party dependencies must be correctly provided for CI to pass.

[`cargo-about`](https://github.com/EmbarkStudios/cargo-about) is used to
automatically comply with open source licenses.

### Troubleshooting cargo-about

- Is it showing a `no license specified` error for a crate you've created? If
  so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so,
  first determine what license the project has and whether this system is
  sufficient to comply with this license's requirements. If you're unsure, ask a
  lawyer. Once you've verified that this system is acceptable add the license's
  SPDX identifier to the `accepted` array in `script/licenses/licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a
  clarification field at the end of `script/licenses/licenses.toml`, as
  specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

### Credits

- SVG icon by Selina Bauder: https://openmoji.org/library/emoji-1F409/#variant=black
