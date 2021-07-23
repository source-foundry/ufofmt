# ufofmt

A highly opinionated UFO source file formatter based on the [Norad library](https://github.com/linebender/norad)

## Installation

```
$ git clone https://github.com/source-foundry/ufofmt.git
$ cd ufofmt && cargo install
```

This installation process installs the `ufofmt` executable.

## Usage

Pass one or more UFO source directory paths to the `ufofmt` executable:

```
$ ufofmt [UFO PATH 1] ... [UFO PATH N]
```

To view total execution duration data, include the `--time` flag:

```
$ ufofmt --time [UFO PATH 1] ... [UFO PATH N]
```

## License

[Apache License v2.0](LICENSE)
