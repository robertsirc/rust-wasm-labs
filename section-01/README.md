# Section One Setup and Configuration

## Introduction

In this section will will be installing Rust and configuring our development environment.

## IDE

For ease of use the labs will be using VS Code as the IDE of choice. If you want to use `VIM` or `nano` by all means you can.

1. Download and install [VS Code](https://code.visualstudio.com/)

## Plugins

These are some very helpful plugin for VS Code that assist with developing in Rust. If you are not using VS Code or don't want to install these plugin's you can skip this section.

1. [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
2. [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

## Installing Rust

The get started guide for Rust will go into a bit more detail about installation but for most cases it will be the following if you are using a Mac or Linux based machine:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you are on a Windows machine please check on the following on how to [install](https://forge.rust-lang.org/infra/other-installation-methods.html) Rust.

If you have Rust installed you are ahead of the game feel just to update Rust:

```bash
rustup update
```

Verify everything is there and working:

```bash
rustup -h
```

```bash
cargo -h
```

## OSX Dependencies

In order to use `cargo-generate` you will need to add the Xcode tool set. If it isn't installed through Xcode the following command will give you the dependencies needed:

```bash
xcode-select --install
```

## Conclusion

Very quick installation and configuration of a development environment for Rust.
