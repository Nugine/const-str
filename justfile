fmt:
    cargo fmt --all

check:
    cargo check --all-features
    cargo clippy --all-features -- -D warnings

test:
    cargo test
    cargo test --all-features
    cargo test --release
    cargo test --all-features --release

dev: fmt check test miri

doc:
    RUSTDOCFLAGS='--cfg docsrs' cargo doc --open --no-deps --all-features

miri:
    cargo +nightly miri test
    cargo +nightly miri test --all-features

sync-version:
    #!/bin/bash -e
    cargo set-version -p const-str-proc-macro   '0.5.4'
    cargo set-version -p const-str              '0.5.5'

publish:
    # cargo publish -p const-str-proc-macro
    cargo publish -p const-str
