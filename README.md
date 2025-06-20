# Goldbach's Conjecture Solver

A Rust program that attempts to solve Goldbach's Conjecture, which states that every even integer greater than 2 can be expressed as the sum of two prime numbers.

## Dependencies

- Rust 1.56.0 or later

## Usage

1. Download the latest binary from the [releases page](https://github.com/dotunwrap/goldbachs-conjecture-solver/releases)
2. Run the program: `./goldbachs-conjecture-solver -n <n>` where `<n>` is the upper bound to test

You can find optional arguments using the `-h` or `--help` flag.

## Building from source

1. Clone the repository: `git clone https://github.com/dotunwrap/goldbachs-conjecture-solver.git`
2. Build the project: `cargo build`
3. Run the program: `cargo run`

You can optionally build a release version using the `--release` flag. The binary will be placed in the `target/release` directory.

## Contributing

Pull requests and issues welcome!
