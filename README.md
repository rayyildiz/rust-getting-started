# Rust Getting Started Project

[![Build](https://github.com/rayyildiz/rust-getting-started/actions/workflows/build.yml/badge.svg)](https://github.com/rayyildiz/rust-getting-started/actions/workflows/build.yml)

This is a comprehensive project designed to guide you through the journey of learning Rust programming from scratch. It
provides a hands-on approach to learning new concepts and applying them via practical coding exercises.

## Folder Structure

The project is structured into several directories, each focusing on a different area of Rust:

- [`examples`](./examples): This directory contains files demonstrating basic Rust features.
- [`problems`](./problems): In this directory, you will find coding problems solved exclusively in Rust. It provides
  some examples to understand how Rust can handle real-world programming challenges.
- [`advanced`](./advanced): This directory dives deeper into Rust by exploring advanced concepts
  like [channels](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html),
  [concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html),
  [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html),
  [actor model](https://github.com/actix/actix), and more.

## Setup Instructions

Before diving into the project, ensure you have Rust correctly installed on your machine. Follow these steps to get that
setup:

1. **Installing Rustup**: Rustup is a toolchain installer for the Rust programming language. Download and
   install it from the [official site](https://rustup.rs/).

2. **Add Rust to your System PATH**: After you've installed Rustup, ensure the PATH variable on your system is updated
   to include the `.cargo/bin` directory.

3. **Verify Installation**: Open a new terminal session and run the command `rustc --version`. If Rust is correctly
   installed, the version number will be displayed.

4. **Update Rust**: It's always a good idea to have the latest stable version. You can update your Rust version by
   running `rustup update` in the terminal.

5. **IDE Setup**: Lastly, for development, you can use an IDE
   like [IntelliJ IDEA](https://www.jetbrains.com/idea/download/) with [Rust Plugin](https://plugins.jetbrains.com/plugin/22407-rust) or
    [Rust Rover](https://www.jetbrains.com/rust)
   or [VS Code](https://code.visualstudio.com/download) with
   the [Rust Analyzer Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Contributing

This project welcomes contributions from everyone. Feel free to open an issue or submit a pull request if you want to
improve or add new features to the project.

Happy coding in Rust! 