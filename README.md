<p align="center">
  <img width="480" height="144" src="https://raw.githubusercontent.com/robertcorponoi/graphics/master/godot-rust-cli-upgrader/logo/godot-rust-cli-upgrader-logo.png">
</p>

<h1 align="center">Godot Rust CLI Upgrader</h1>

<p align="center">A CLI tool to help you upgrade your Godot Rust CLI project between versions that introduce breaking changes.</p>

[![Build Status](https://www.travis-ci.com/robertcorponoi/godot-rust-cli-upgrader.svg?branch=main)](https://www.travis-ci.com/robertcorponoi/godot-rust-cli-upgrader)
![Crates.io](https://img.shields.io/crates/v/godot-rust-cli-upgrader)
![Crates.io](https://img.shields.io/crates/d/godot-rust-cli-upgrader)
![Crates.io](https://img.shields.io/crates/l/godot-rust-cli-upgrader)
[![Discord](https://img.shields.io/discord/853728834519040030.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/kr9EkBp7)

## Documentation

Note: If you have any questions about how to use this CLI tool or want to report a bug, feel free to open an issue in the GitHub or the [Discord](https://discord.gg/kr9EkBp7).

Also keep in mind that the main branch will usually be ahead of the version on [crates.io](https://crates.io/crates/godot-rust-cli-upgrader).

**Table of Contents**

- [Introduction](#introduction)
- [Installation](#installation)
- [Step By Step](#step-by-step)
- [API](#api)
    - [upgrade](#upgrade)
    - [version](#version)
- [Questions](#questions)
- [License](#license)

## Introduction

Since [Godot Rust CLI](https://github.com/robertcorponoi/godot-rust-cli) is going through breaking changes pre-v1.0.0, Godot Rust CLI Upgrader was created to help alleviate some of the pain from upgrading between versions. 

## Installation

To install Godot Rust CLI Upgrader, use:

```sh
cargo install godot-rust-cli-upgrader
```

To upgrade your version of Godot Rust CLI Upgrader to the latest version, use:

```sh
cargo install --force godot-rust-cli-upgrader
```

### Step-by-Step

This guide will walk you through upgrading your Godot Rust CLI library to the latest version. If you would rather just look at the API, check it out [below](#api).

1. The first step is to navigate to the library created by Godot Rust CLI. This should be the directory with the configuration file (either `project.toml`, `godot-rust-cli.toml`, or `godot-rust-cli.json`).

2. Figure out what version you want to upgrade to. This should always be the latest version, which at the time of writing this is v0.3.x, but you have a choice to upgrade to a different version. For this guide, we'll be upgrading to to v0.3.x.

**Note:** Godot Rust CLI Upgrader works with major and minor versions but it doesn't care about patches. The only reason we even use minor versions is because pre-v1.0.0 there's going to be breaking changes. This means that when we upgrade, we specify the patch as x like "0.2.x" or "0.3.x".

3. Now that we've decided we want to upgrade to version 0.3.x, we can use the upgrade command like so:

```sh
godot-rust-cli-upgrader upgrade "0.3.x"
```

4. At this point the CLI should notify you of the steps it is taking until it is done. You'll notice that if you're upgrading through multiple versions, like from 0.1.x to 0.3.x, it'll upgrade the library from 0.1.x to 0.2.x first, and then from 0.2.x to 0.3.x. When it's finished you should be able to use your library with the latest version Godot Rust CLI.

## API

### upgrade

Upgrades the version of the library to be compatible with the specified version of Godot Rust CLI.

**Usage:**

```sh
godot-rust-cli-upgrader upgrade <version>
```

where:

- `version` is the version to upgrade the library to. Currently the available versions are: `"0.2.x"` and `"0.3.x"`.

**Examples:**

Upgrading the library to version `"0.3.x"`:

```sh
godot-rust-cli-upgrader upgrade "0.3.x"
```

### version

Returns the version of Godot Rust CLI Upgrader.

**Usage:**

```sh
godot-rust-cli-upgrader --version
```

## Questions

Check out the [Discord](https://discord.gg/kr9EkBp7) to ask any questions or concerns about the cli or Godot + Rust in general.

## License

[MIT](./LICENSE)
