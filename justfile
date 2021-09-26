fmt:
    cargo fmt --all

check:
    cargo check --all-features
    cargo clippy --all-features -- -D warnings

test:
    cargo test --all-features

dev: fmt check test

doc:
    cargo doc --open --no-deps --all-features

miri:
    cargo +nightly miri test --all-features
