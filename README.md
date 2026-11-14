# tracing-dinhani

Installs a `tracing` subscriber with one fixed format: pipe-separated columns with local time with milliseconds, the level, the message padded to 20 characters, then each field as a bold name and a magenta value.

This is for personal use: it is the format I like, packaged so my scripts do not repeat the setup. There is nothing special about it; for any other format, configure `tracing-subscriber` directly.

```
10:04:24.916 | INFO  | Finished             | scanned=2 stripped=2 saved=1040 saved_mb=0.6
```

## Install

```toml
[dependencies]
tracing-dinhani = { git = "https://github.com/dinhani/rust-tracing-dinhani", tag = "v0.1.0" }
tracing = "0.1"
```

## Usage

```rust
use tracing::info;

tracing_dinhani::init();
info!(scanned = 2, "Finished");
```

`init` installs the global subscriber, so it is called once, at the start of `main`.

The level filter comes from `RUST_LOG` (`RUST_LOG=info,oxipng=warn`). Without it, the filter is INFO.
