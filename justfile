fmt:
    cargo fmt --all

check:
    cargo check --all-features
    cargo clippy --all-features -- -D warnings

test:
    cargo test --all-features

dev: fmt check test

