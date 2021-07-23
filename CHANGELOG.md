# Changelog

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
