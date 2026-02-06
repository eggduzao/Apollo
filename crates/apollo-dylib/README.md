# Apollo dynamic library

```toml
# Cargo.toml
[workspace.dependencies.apollo]
package = "apollo-dylib"
```

```toml
# .cargo/config.toml
[build]
rustflags = [
  "-C",
  "prefer-dynamic",
]
```
