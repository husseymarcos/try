## Trust

Trust is a small command‑line tool that reimagines the original [`try`](https://github.com/tobi/try) project in Rust.

Where `try` helps you quickly create and jump into fresh folders for your experiments, Trust aims to offer the same playful, low‑friction workflow, but built with Rust instead of Ruby.

### What this project is about

- **Same spirit, new implementation**: This is a ground‑up rewrite of `try` using Rust, keeping the original idea and overall feel.
- **Focused on experiments**: The goal is still to give every little idea or throwaway project a tidy home, so your experiments are easy to find later.
- **Simple to use**: The interface is designed to stay approachable, even if you are not familiar with Rust or programming details.

### Running the spec tests

The repository ships with a spec test suite under `spec/tests` that can be run against any `trust` binary.

1. **Build the binary** (from the project root):

```bash
cargo build --bin trust
```

2. **Run the spec runner** against the built binary:

```bash
./spec/tests/runner.sh ./target/debug/trust
```