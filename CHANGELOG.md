# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add AUR installation instructions to README (#38) by @nerdyslacker
- Add binary aur install instructions to README (#53) by @bananas
- Add perl-Time-Piece to linux script (#1) by @LHolten
- Built in language support for OpenTofu (#33) by @theDoctor
- Make number types in general editable
- Options to adjust client side decoration rounding/shadow (#20)

### Fixed

- Updated documentation
- Wasm is not an acronym
- Add zlib-ng-compat-static as dependency for Fedora build (#62) by @voidedgin
- Documentation around installing extensions. (#23) by @edwardloveall
- Extend icon theme docs with enable settings (#6) by @Petrosz007
- Fix LSP github download logic for pre-release (#67) by @Petrosz007
- Fix Supertab not performing word completion
- Fix `block_comment` and `documentation_comment` for Rust (#24) by @fzzr
- Fix superhtml LSP (#56) by @Petrosz007
- Fixed package name for cmake for Gentoo packages (#28) by @stepanov
- Fixes compilation issue on aarch64-linux (#61) by @voidedgin
- Highlight that Rust in required to install some extensions (#34) by @ash-sykes
- Make UI and Buffer Font match. (#69) by @voidedgin
- Modify single instance port numbers to not clash with Zed (#10)
- Fixed typos in README (#19) by @GulfSugar
- extensions: Use system clang, if it supports the wasm target (#64) by @selfisekai
- Enable the uninstall extension button (#32)
- Fix extensions being installed to tmp dir (#26)

### Changed

- Bump crash-handler to 0.7 (#39) by @selfisekai
- Update wild to 0.8.0 (#63) by @voidedgin

## [1.0.0] - 2026-03-01

### Added

- First release
- Website: <https://gram.liten.app>
