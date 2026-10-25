# Plush size helper

Type a plush height in cm. It prints the height in inches and the fabric you need for a simple square pillow of that size: two squares cut side by side, with 1 cm seam allowance on every edge.

```
$ cargo run -- 30
30 cm = 11.81 in
Pillow fabric: 32 x 64 cm
```

Add `in` for inches or `mm` for millimetres: `cargo run -- 12in`, `cargo run -- 300mm`.

Change the seam allowance (default 1 cm) with `--seam`: `cargo run -- 30 --seam 1.5`.

Make a rectangle pillow with `--width`: `cargo run -- 30 --width 40`.

A height that is missing, not a number, zero or negative stops with a usage message and exit code 2.

## Tests

```
cargo test
```

## How this repo is run

This is a practice repo for [qew](PRACTICE.md), a work ledger: every change starts as a qew job and lands through a pull request.
