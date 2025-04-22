# disk-thrash

A simple Rust application.

## Description

This application is a simple Rust program that utilizes multiple CPU cores to perform some task.

## Dependencies

- num_cpus = "1.16.0"
- rand = "0.9.1"
- uuid = { version = "1.16.0", features = ["v4"] }

## Installation

Install the [Rust toolchain](https://rustup.rs) first.

```
cargo install --git https://github.com/pcgeek86/disk-thrash.git
```

## Usage

To build and run the application, you need to have Rust installed.

```bash
cargo build --release
./target/release/disk-thrash