#####
date = "2024-01-22"
author = "Nzuzo Magagula"
summary = "Learn how to set up your Rust development environment with rustup, cargo, and your favorite IDE."
topics = ["Rust", "Setup", "Development Environment"]
tags = ["rust", "beginner", "setup", "tooling"]
thumbnail = "https://www.rust-lang.org/static/images/rust-logo-blk.svg"
reading_time = "10 min"
category = "Tutorial"
#####

# Setting Up Your Rust Development Environment

Before we can start coding in Rust, we need to set up our development environment. This guide will walk you through installing Rust and configuring your IDE.

## Installing Rust

The easiest way to install Rust is through `rustup`, the official Rust toolchain installer.

### On Linux and macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### On Windows

Download and run the installer from [rustup.rs](https://rustup.rs/)

## Verifying Installation

After installation, verify that Rust is installed correctly:

```bash
rustc --version
cargo --version
```

## Your First Rust Program

Create a new project with Cargo:

```bash
cargo new hello_rust
cd hello_rust
```

This creates a new directory with the following structure:
```
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

## Running Your Program

```bash
cargo run
```

You should see "Hello, world!" printed to the console.

## Recommended IDEs

1. **VS Code** with rust-analyzer extension
2. **IntelliJ IDEA** with Rust plugin
3. **Vim/Neovim** with rust.vim

In the next article, we'll dive into Rust's ownership system!
