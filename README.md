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

After installing the application, run it with:

```
disk-thrash
```

To specify a parent directory for the temporary files:

```
disk-thrash --parent-dir /path/to/parent/directory
```