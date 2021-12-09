# Beam
[![Crates.io](https://img.shields.io/crates/v/beamcli)](https://crates.io/crates/beamcli)

> Beam me up Ferris!

## What is Beam?

Beam an interface on top of the Teleport CLI. It uses skim, a fuzzy finder written in Rust, to provide a nice interface for searching and filtering.

## Table of Contents
- [Beam](#beam)
  * [What is Beam?](#what-is-beam-)
  * [Table of Contents](#table-of-contents)
  * [Installation](#installation)
  * [Configuration](#configuration)
    + [Caching](#caching)
  * [Usage](#usage)
    + [Search Syntax](#search-syntax)


## Installation

> Make sure that you have the [Teleport CLI](https://goteleport.com/docs/installation/) installed, before using Beam.

For installing you will have to install Rust. [Rustup](https://rustup.rs/) is the recommended way to do that.  
You can install beam through running:

```bash
rustup default nightly && rustup update
```

```bash
cargo install beamcli
```


## Configuration

Before using Beam you will have to configure the Teleport proxy.

```bash
beam config set --proxy teleport.example.com
```

Beam will automatically use the user, from which you are running the command, as the username for connecting to a host.
To use a different user, you can use the `--user` flag, or configure a new default using the following command:

```bash
beam config set --user myuser
```

You can also specify a list of labels that will explicitly be shown. If you don't specify any, Beam will show all labels.

```bash
beam config set --label-whitelist environment application
```


### Caching

By default Beam caches the list of nodes it receives from Teleport for 24 hours. To avoid using cache you can use the `--clear-cache` or `-c` flag:
```bash
$ beam -c
```
You can change the cache duration using the `--cache-ttl` flag.
The following example will cache the list of nodes for 1 hour:
```bash
$ beam config set --cache-ttl 3600
```

## Usage

A few useful Beam commands:

1. Opening a fuzzy finder view for selecting a host:
```bash
$ beam
```
2. Listing the names of all available nodes
```bash
$ beam list --format names
host1.example.com
host2.example.com
```
3. Directly connect to a host via its hostname
```bash
$ beam connect server.example.com
```

### Search Syntax

Beam uses skim under the hood for its fuzzy search. The syntax for searching is the same as for skim.
See [skim](https://github.com/lotabout/skim) for more information.

| Token    | Match type                 | Description                       |
|----------|----------------------------|-----------------------------------|
| `text`   | fuzzy-match                | items that match `text`           |
| `^music` | prefix-exact-match         | items that start with `music`     |
| `.mp3$`  | suffix-exact-match         | items that end with `.mp3`        |
| `'wild`  | exact-match (quoted)       | items that include `wild`         |
| `!fire`  | inverse-exact-match        | items that do not include `fire`  |
| `!.mp3$` | inverse-suffix-exact-match | items that do not end with `.mp3` |

`skim` also supports the combination of tokens.

- Whitespace has the meaning of `AND`. With the term `src main`, `skim` will search
    for items that match **both** `src` and `main`.
- ` | ` means `OR` (note the spaces around `|`). With the term `.md$ |
    .markdown$`, `skim` will search for items ends with either `.md` or
    `.markdown`.
- `OR` has higher precedence. So `readme .md$ | .markdown$` is grouped into
    `readme AND (.md$ OR .markdown$)`.