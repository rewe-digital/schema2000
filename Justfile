# run tests/checks that are also run by github actions
ci:
    cargo fmt --all -- --check
    cargo check --tests --examples
    cargo test
    cargo clippy -- -D warnings

# run clippy in pedantic mode (but warnings only)
clippy_pedantic:
    cargo clippy --all-targets -- -W clippy::pedantic

# run clippy (from nightly) in pedantic mode (but warnings only).
# you need to install nightly via rustup, e.g.: `rustup toolchain install nightly --component clippy`
clippy_nightly:
    cargo +nightly clippy --all-targets -- -W clippy::pedantic
