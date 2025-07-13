dev:
    just fmt
    just lint
    just test
    just unstable-test
    just miri

fmt *ARGS:
    cargo fmt --all {{ARGS}}

lint *ARGS:
    cargo clippy --all-features --tests --benches {{ARGS}}

test *ARGS:
    cargo test {{ARGS}}
    cargo test --features all {{ARGS}}
    cargo test --release {{ARGS}}
    cargo test --release --features all {{ARGS}}

unstable-test *ARGS:
    cargo test --all-features {{ARGS}}
    cargo test --all-features --release {{ARGS}}

miri *ARGS:
    cargo +nightly miri test --all-features {{ARGS}}

doc *ARGS:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features {{ARGS}}

ci:
    just fmt --check
    just lint -- -D warnings
    just test
    just miri

sync-version:
    cargo set-version -p const-str-proc-macro   '0.6.3'
    cargo set-version -p const-str              '0.6.3'
