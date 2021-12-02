# Beam
[![Crates.io](https://img.shields.io/crates/v/beamcli)](https://crates.io/crates/beamcli)
## What is Beam?

Beam an interface on top of the Teleport CLI. It uses skim, a fuzzy finder written in Rust, to provide a nice interface for searching and filtering.

## Usage

A few useful Beam commands:

1. Listing all available nodes and ignoring cache:
```bash
beam -c
```
2. Listing the names of all available nodes
```bash
beam list --format names
```
3. Directly connect to a host via its hostname
```bash
beam connect server.example.com
```
## Installation

> Make sure that you have the [Teleport CLI](https://goteleport.com/docs/installation/) installed, before using Beam.

For installing you will have to install Rust. [Rustup](https://rustup.rs/) is the recommended way to do that.  
You can install beam through running:

```bash
cargo install
```
