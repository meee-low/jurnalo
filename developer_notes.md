## One small fragility in the build:

schema.rs is written automatically by Diesel when it runs migrations (builds the database). Because of that, we need to be at the latest migration
(i.e. not have reverted/never run them) _at compile time_, because we use schema.rs as an actual rust module, to get the schema. models.rs depends on schema.rs being on the right version, or you'll get compilation errors and not build.

Current initial/reset build process:

```
diesel migration redo &&
diesel migration run &&
cargo run
```

After the first build, as long as the migrations are at the latest, you can just run `cargo build` or `cargo run` as normal. The initial cargo run is useful for populating the database with some "test data" loaded from [mockdb/toml_test.toml](mockdb/toml_test.toml)

If you don't have the diesel command, you need to install it:

[https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)

[https://github.com/diesel-rs/diesel/tree/master/diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)

Suggested:

```
cargo install diesel_cli --no-default-features --features sqlite
```
