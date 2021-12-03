# Beam
[![Crates.io](https://img.shields.io/crates/v/beamcli)](https://crates.io/crates/beamcli)

> Beam me up Ferris!

## What is Beam?

Beam an interface on top of the Teleport CLI. It uses skim, a fuzzy finder written in Rust, to provide a nice interface for searching and filtering.

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
