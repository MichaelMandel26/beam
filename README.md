# Beam
[![Crates.io](https://img.shields.io/crates/v/beamcli)](https://crates.io/crates/beamcli)
[![CI](https://github.com/MichaelMandel26/beam/actions/workflows/main.yml/badge.svg)](https://github.com/MichaelMandel26/beam/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/MichaelMandel26/beam/branch/main/graph/badge.svg?token=QAYMC9JTCZ)](https://codecov.io/gh/MichaelMandel26/beam)

> Beam me up Ferris!

## What is Beam?

Beam is an interface on top of the Teleport CLI. It uses skim, a fuzzy finder written in Rust, to provide a nice interface for searching and filtering.

# Table of contents

- [Beam](#beam)
  - [What is Beam?](#what-is-beam)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Through Brew](#through-brew)
    - [Through Cargo](#through-cargo)
  - [Configuration](#configuration)
    - [Pattern matching](#pattern-matching)
    - [Caching](#caching)
    - [Port forwarding](#port-forwarding)
  - [Usage](#usage)
    - [Search Syntax](#search-syntax)
  - [Adding completions to your shell](#adding-completions-to-your-shell)



## Installation

### Through Brew

For installing Beam through Homebrew, run the following commands:

```bash
brew tap MichaelMandel26/beamcli
brew install beam
```

> This will also automatically install the [Teleport CLI](https://goteleport.com/docs/installation/), as it is a dependency of Beam.

### Through Cargo

> Make sure that you have the [Teleport CLI](https://goteleport.com/docs/installation/) installed, before using Beam through cargo.

For installing Beam through cargo you will have to install Rust. [Rustup](https://rustup.rs/) is the recommended way to do that.  
You can then install it through running:

```bash
cargo install beamcli
```

## Configuration

Before using Beam you will have to configure a default profile.

```bash
$ beam configure
✔ Profile name · myProfile
✔ Do you want to auto-select this profile, using a regex pattern on the hostname? · no
✔ Proxy · teleport.example.com
✔ Username · dzefo
✔ Authentication Method · default
✔ Cache TTL · 86400
✔ Do you want to only show specific labels? · no
```

If you want to use SSO as your authentication method, you will have to set `sso` for `Authentication Method`

For only showing specific labels, you can set `yes` for `Do you want to only show specific labels?`
```bash
$ beam configure
...
✔ Do you want to only show specific labels? · yes
✔ Label · env
✔ Add another label? · yes
✔ Label · application
✔ Add another label? · no
```

### Pattern matching

If you want to select a specific profile, for a set of hostnames, that match a specific pattern, you can specify `yes` for `Do you want to auto-select this profile, using a regex pattern on the hostname?`

```bash
$ beam configure
Configuring profile dev-profile:
✔ Do you want to auto-select this profile, using a regex pattern on the hostname? · yes
✔ Regex Pattern for auto-selecting profile · \b(quality|staging)\b.*
...
```

Beam will then match any of the following hostnames:
- quality.app.example.com
- staging.app.example.com

If the hostname doesnt match any profile pattern, Beam will use the default profile.

> Important: Beam will only use the proxy from the default profile, when running the `beam` command, as it does not know the hostname, before selecting it. When using the `beam connect <hostname>` command, Beam will use the hostname from the command line and is able to use the proxy from the profile.

### Caching

By default Beam caches the list of nodes it receives from Teleport for 24 hours. To avoid using cache you can use the `--clear-cache` or `-c` flag:
```bash
$ beam -c
```
You can change the cache duration using the `Cache TTL` option.
The following example will cache the list of nodes for 1 hour:
```bash
$ beam configure
...
✔ Cache TTL · 3600
```

### Port forwarding

If you want to forward a specifc port to your localhost, you can add the following attributes to one of your profiles. 

```toml
[profile.mysql]
...
enable_port_forwarding = true
listen_port = 3306
remote_host = "127.0.0.1"
remote_port = 3306
```

using this feature you could for example forward the port `3306` every time you connect to a node containing the word `mysql`. This would enable you to inspect a database running on the node using a database management desktop application

```toml
[profile.mysql]
default = false
proxy = "teleport.example.com"
username = "firstname.lastname"
auth = "sso"
cache_ttl = 86400
enable_port_forwarding = true
host_pattern = "(._mysql._)"
listen_port = 3306
remote_host = "127.0.0.1"
remote_port = 3306
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
4. Manually selecting a profile to use
```bash
$ beam --profile myProfile
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

## Adding completions to your shell

In order to add completions to your shell, you can use one of the following commands:

```bash
$ beam completions zsh > ~/.zfunc/_beam
```

```bash
$ sudo apt install bash-completions
$ mkdir -p ~/.local/share/bash-completion/completions
$ beam completions bash > ~/.local/share/bash-completion/completions/beam
```

```bash
$ mkdir -p ~/.config/fish/completions
$ beam completions fish > ~/.config/fish/completions/beam.fish
```
