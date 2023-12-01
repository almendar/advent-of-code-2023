# Advent of Code 2023 Rust

## Instructions

**To add a new day**

```bash
cargo new day1
```

**Running day**

```bash
cargo run -p day1
```

**Running Tests**

```
cargo test -p day1
```

**Test everything**

```
cargo test
```

**To add common lib to each day**


Add to `Cargo.uml` in each day:
```toml
[dependencies]
common = { path = "../common" }
```