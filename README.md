# ufofmt

![crates.io](https://img.shields.io/crates/v/ufofmt.svg)
[![stable toolchain unit tests](https://github.com/source-foundry/ufofmt/actions/workflows/stable-unittests.yml/badge.svg)](https://github.com/source-foundry/ufofmt/actions/workflows/stable-unittests.yml)
[![beta toolchain unit tests](https://github.com/source-foundry/ufofmt/actions/workflows/beta-unittests.yml/badge.svg)](https://github.com/source-foundry/ufofmt/actions/workflows/beta-unittests.yml)

A fast, flexible UFO source file formatter based on the Rust [Norad library](https://github.com/linebender/norad)

## About

ufofmt is a Rust executable that supports customizable UFO source file formatting.

### Default source file format

  | glif | plist | fea
-- | -- | -- | --
line endings | line feed | line feed | line feed
indentation spacing | single tab per level | single tab per level | n/a
XML declaration attributes | double quotes | double quotes | n/a

Custom formatting options are described in the Usage section below.

## Installation

The installation process installs the `ufofmt` executable.

[Install Rust](https://www.rust-lang.org/tools/install), then follow the instructions below.

### User installation

The following command installs the latest release build:

```
$ cargo install ufofmt
```

Upgrade a previous installation to a new release version with:

```
$ cargo install --force ufofmt
```

### Developer installation

The following command installs a build from the latest commit in the main branch of the repository:

```
$ git clone https://github.com/source-foundry/ufofmt.git
$ cd ufofmt && cargo install --path .
```

## Usage

Pass one or more UFO source directory paths to the `ufofmt` executable:

```
$ ufofmt [OPTIONS] [UFO PATH 1] ... [UFO PATH N]
```

Use the command `ufofmt --help` to view all available command line options.

### Custom source formatting options

#### Indentation spacing character type

Single tab indentation spacing per level is the default.  Switch to space characters with the `--indent-space` command line option.  See the section below to define the number of indentation spacing characters per level.

#### Indentation spacing character number per level

Define between 1 - 4 tab or space indentation chars with the `--indent-number [NUMBER]` command line option. See the section above to use spaces instead of tabs.

#### XML declaration quote style

XML declaration attributes are enclosed in double quotes by default.  Convert to single quotes with the `--singlequotes` command line option.

## Contributing

Contributions to the project are welcomed!  All contributions are accepted under the project license defined in the License section below.

### Source contributions

Test local changes in the executable with:

```
$ cargo run -- [ARGS]
```

Add tests to cover your source changes and run the test suite locally with:

```
$ cargo test
```

Please open a GitHub pull request with your change proposal.

### Documentation contributions

Please build and review your documentation changes locally with:

```
$ cargo doc --open
```

Please open a GitHub pull request with your change proposal.

## License

[Apache License v2.0](LICENSE)
