## One small fragility in the build:

Since schema.rs changes whenever we run migrations, we need to be at the latest migration
(i.e. not have reverted them) *at compile time*, because we use that as a module to get the schema.

Current: build process
```
diesel migration run
cargo run --bin backend
cargo run {args}
```