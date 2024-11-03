# ent - recipe tree management

`ent` is a tool for working with packaging recipes in git trees/monorepos.

Primarily it is intended for use with [Serpent OS](https://serpentos.com) packaging, and to
a limited degree, [Solus](https://getsol.us) packaging.

## Installation

`ent` is written in Rust, and can be installed via `cargo`:

```sh
cargo install --path .
```

## Usage

`ent` is a command line tool, and can be invoked with `ent`:

```sh
ent --help
```

For example, to check for updates to the recipes in the current directory:

```sh
ent check updates
```

## Copyright

`ent` is licensed under the terms of the MPL-2.0 license. See the [LICENSE](LICENSE) file for details.
