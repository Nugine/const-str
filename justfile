fmt:
    cargo fmt --all

check:
    cargo check --all-features
    cargo clippy --all-features -- -D warnings

test:
    cargo test
    cargo test --all-features

dev: fmt check test

doc:
    RUSTDOCFLAGS='--cfg docsrs' cargo doc --open --no-deps --all-features

miri:
    cargo +nightly miri test --all-features

sync-version:
    #!/bin/bash -e
    vers='0.4.2'
    pushd const-str-proc-macro
    cargo set-version $vers
    popd
    pushd const-str
    cargo set-version $vers
    popd
