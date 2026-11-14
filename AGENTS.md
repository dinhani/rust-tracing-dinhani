# rust-tracing-dinhani

Rust library that installs a tracing subscriber with one fixed format.

- Every release bumps `version` in `Cargo.toml` and gets a matching `v<version>` tag.
- The format has no options; `init()` is the whole API.
- Every function, struct and enum has a one-line `///` doc comment.
- `just lint` passes before every commit.
