# apollo-sql

`apollo-sql` is a sub-crate of the [Apollo](https://crates.io/crates/apollo) library, offering a SQL
transpiler. It allows for SQL query conversion to Apollo logical plans.

## Usage

To use `apollo-sql`, add it as a dependency to your Rust project's `Cargo.toml` file:

```toml
[dependencies]
apollo-sql = "0.30.0"
```

You can then import the crate in your Rust code using:

```rust
use apollo_sql::*;
```

**Important Note**: This crate is **not intended for external usage**. Please refer to the main
[Apollo crate](https://crates.io/crates/apollo) for intended usage.
