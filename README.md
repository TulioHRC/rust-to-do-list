# rust-to-do-lis

## First Section: Members

- Tiago Gonçalves Laranjo
- Túlio Henrique Rodrigues Costa

## Second Section: Explanation of the system

A basic system to organize your to-do list, using only the terminal. Motivated by a university project, based on software testing.

### Features

1. Add tasks
2. Delete tasks
3. Complete tasks

## Third Section: Tecnologies used

This system was made using Rust, and its own testing framework.

Also used libraries:

- clap
- rusqlite
- chrono

# Requirements

- cargo

# How to run the system?

- cargo build
- cargo run help
- cargo run `commands`

Obs.: cargo run help <command-name>, tells you how to use the command.
Obs.2: cargo doc, generates documentation
Obs.3: cargo build --release, generates a .exe file at ./target/release/rust-to-do-list

# How to run the test?

- cargo run test

Create coverage report:

- cargo tarpaulin --out Html