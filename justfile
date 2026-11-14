set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

[doc("List recipes")]
[group("project")]
default:
    @just --list --unsorted

[doc("Check format, lints")]
[group("project")]
lint:
    cargo +nightly fmt --check
    cargo clippy --all-targets -- -D warnings

[doc("Build library")]
[group("run")]
build:
    cargo build

[doc("Run tests")]
[group("test")]
test:
    cargo test
