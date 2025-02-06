# Rust OS Tutorial

This repository contains a minimal operating system kernel written in Rust. It is inspired by the [Writing an OS in Rust](https://os.phil-opp.com/) tutorial by Philipp Oppermann.

## Features

- A minimal kernel written in Rust
- Boots on real hardware and in virtual machines
- Support for printing to the screen using VGA text mode
- A focus on teaching low-level programming concepts in Rust
- memory allocation, fixed size, linked list etc.
- cpu exceptions, page faults, double faults

## Requirements

Before building and running the kernel, ensure you have the following installed:

- **Rust nightly compiler**: Install Rust and switch to the nightly toolchain using:
  ```bash
  rustup install nightly
  rustup default nightly
