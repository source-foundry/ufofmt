# Changelog

## v0.6.0

- add consistent cross-platform line feed line ending serialization across plist, glif, and feature files
- Dependency updates:
  - bump libc v0.2.101 -> v0.2.102

## v0.5.0

- add custom indentation spacing support (1 - 4 tabs or spaces)
- Dependency updates:
  - bump ctor v0.1.20 -> v0.1.21
  - bump proc-macro2 v1.0.28 -> v1.0.29
  - bump serde v1.0.129 -> v1.0.130
  - bump serde_derive v1.0.129 -> v1.0.130
  - bump structopt v0.3.22 -> v0.3.23
  - bump syn v1.0.75 -> v1.0.76

## v0.4.1

- Dependency updates (includes bump beyond yanked version of crossbeam-deque):
  - bump bitflags v1.2.1 -> v1.3.2
  - bump crossbeam-deque v0.8.0 -> v0.8.1
  - bump libc v0.2.98 -> v0.2.101
  - bump memchr v2.4.0 -> v2.4.1
  - bump proc-macro2 v1.0.27 -> v1.0.28
  - bump serde v1.0.126 -> v1.0.129
  - bump serde_derive v1.0.126 -> v1.0.129
  - bump syn v1.0.74 -> v1.0.75
  - bump xml-rs v0.8.3 -> v0.8.4

## v0.4.0

- add parallel glif serialization support
- add custom outpath file extensions with `--out-ext` option
- add custom outpath appended file name strings with `--out-name` option
- add optional XML declaration single quote formatting with `--singlequote` option
- add lazy_static dependency
- bump norad dependency to v0.5.0 (from v0.4.0)
- add pretty_assertions developer dependency

## v0.3.0

- activate rayon feature in norad library dependency
- add new lib module
- refactor error handling to lib sub-modules
- check UFO dir path validity during format execution to avoid a separate loop
- add unit tests
- add `fs_extra` dev dependency (unit testing)
- add `tempdir` dev dependency (unit testing)
- add Mutator Sans sources to support testing (MIT License)
- add rustdoc user documentation
- add developer documentation on the repository README.md

## v0.2.0

- add UFO directory validity checks and informative error messages
- refactor UFO dir path argument parsing to use PathBuf, skip unnecessary extra String->PathBuf instantiation step
- refactor format_ufo function to take a PathBuf reference parameter (from String reference)

## v0.1.2

- add Cargo.lock definitions

## v0.1.1

- improve help message for input path arguments
- fix installation documentation (`cargo install` no longer works)

## v0.1.0

- initial release with rayon-based parallel UFO source formatting support
