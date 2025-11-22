# Ludd

[![CI](https://github.com/krig/zod/actions/workflows/run_tests.yml/badge.svg)](https://github.com/krig/zod/actions/workflows/run_tests.yml)

Welcome to Ludd, an attempt to unfuck Ludd and turn it into a _good_ code editor.

- NO AI
- NO TELEMETRY
- NO PROPRIETARY "COLLABORATION"
- NO CLA
- NO TERMS OF USE
- NO THIRD PARTY INTEGRATIONS

---

### Installation

Build it from source. It's a text editor for source code. You can figure it out.

### Developing Ludd

- [Building Ludd for macOS](./docs/src/development/macos.md)
- [Building Ludd for Linux](./docs/src/development/linux.md)
- [Building Ludd for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Ludd.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
