# disk-thrash

A simple disk stress test tool. It continuously creates and deletes files on the filesystem.

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

The default buffer size is 100 MB
To specify a different buffer size for the temporary files:

```
disk-thrash --buffer-size 200
```