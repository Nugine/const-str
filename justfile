dev:
    just fmt
    just lint
    just test
    just unstable-test
    just miri

fmt *ARGS:
    cargo fmt --all {{ARGS}}

lint *ARGS:
    cargo clippy --all-features --all-targets {{ARGS}}

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

coverage *ARGS:
    cargo llvm-cov --all-features --html {{ARGS}}

doc *ARGS:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features {{ARGS}}

ci:
    just fmt --check
    just lint -- -D warnings
    just test
    just miri

sync-version:
    cargo set-version -p const-str-proc-macro   '1.0.0'
    cargo set-version -p const-str              '1.0.0'
