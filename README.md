# Tehanu

[![CI](https://codeberg.org/krig/galadriel/actions/workflows/run_tests.yml/badge.svg)](https://codeberg.org/krig/galadriel/actions/workflows/run_tests.yml)

This project is a strongy opinionated fork of the Zed code editor. The main
rationale behind this fork is that I am curious about a new editor written in
rust, but I don't want anything to do with AI, I don't want my editor to phone
home to an American corporation and I don't want to sign in, sign up or
accept any terms of use to write code.

I have gotten to this point in my life without making the tools of my trade into
a subscription service from some of the worst companies ever to exist, and I
don't intend to start now.

I am making these changes available to all without warranty or

## Manifesto

This project is first and foremost a source code editor. It aims to be a fast,
reliable and hackable tool for developers to use, reuse, share and modify. It
will _never_ contain, support or condone any of the following "features" that
_permeate_ the Zed code editor: AI, Telemetry, Proprietary server components,
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

For more thoughts on this topic, see [MISSION.md][].

---

### Installation

Build it from source. It's a tool for developers; you should at least compile it
yourself.

Make sure you have Rust installed (via rustup, preferrably).

```sh
cargo run --release
```

### Developing

- [Building for macOS](./docs/src/development/macos.md)
- [Building for Linux](./docs/src/development/linux.md)
- [Building for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to this
project.

### Licensing

This project is subject to the licenses of its original sources and
dependencies.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
