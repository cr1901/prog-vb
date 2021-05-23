# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project does _not_ strictly adhere to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Major version
changes are reserved for code restructuring changes. Minor version changes
are reserved for new features. Patch level changes are reserved for bug
fixes in between minor versions.

# `prog-vb` Command Line Virtual Boy Flash Programmer

## [Unreleased]

## [v0.11.0] - 2021-05-22
### Added
- Signify erase time with a message, rather than no output (takes about a minute).
- Add `aarch64-unknown-linux-gnu` and `armv7-unknown-linux-gnueabihf` to the
  build matrix.

### Changed
- Use [eyre] library for error handling.
- Use [calm_io] so that standard streams don't panic.
- Use [Github Actions] for CI instead of Travis.

### Fixed
- CHANGELOG.md's [Unreleased] link now points to the correct URL.

## [v0.10.0] - 2019-03-07
### Added
- Input ROMs are now automatically padded to 2MB if necessary to satisfy
  the address decoding [scheme] of the Virtual Boy system.
- Add version `-v` command-line argument.
- Add check to ensure input ROM meets following conditions:
  - ROM >= 16kB (arbitrary lower limit- actual limit is 1kB).
  - ROM <= 2MB.
  - ROM size is power of two (required due to decoding [scheme]).

### Fixed
- By adding padding support, fix cryptic error message where ROMs < 2MB
  would fail when EOF was reached.
- Use `-v` version command-line argument to test that binaries are loaded
  properly and exit with success code (`0`).

## v0.9.0 - 2019-03-04
### Added
- Minimal FlashBoy (Plus) programmer which uses [hidapi-rs] to program
  Virtual Boy ROMs.
- Use [Travis CI](https://travis-ci.org/cr1901/prog-vb) and
  [Github Releases](https://github.com/cr1901/prog-vb/releases) to support
  x86_64 Windows, MacOS, and Linux binaries.

[eyre]: https://github.com/yaahc/eyre
[calm_io]: https://github.com/myrrlyn/calm_io
[Github Actions]: https://github.com/cr1901/prog-vb/actions
[scheme]: https://www.planetvb.com/modules/newbb/viewtopic.php?post_id=38140#forumpost38140
[hidapi-rs]: https://github.com/ruabmbua/hidapi-rs

[Unreleased]: https://github.com/cr1901/prog-vb/compare/v0.11.0...HEAD
[v0.11.0]: https://github.com/cr1901/prog-vb/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/cr1901/prog-vb/compare/v0.9.0...v0.10.0
