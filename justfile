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

audit:
    cargo audit -D warnings
    cargo deny check bans
    cargo deny check sources
    cargo deny check licenses

coverage *ARGS:
    cargo llvm-cov --all-features --html {{ARGS}}

doc *ARGS:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features {{ARGS}}

ci:
    just fmt --check
    just lint -- -D warnings
    just test
    just miri

# Updates the inherited workspace version and const-str's internal dependency.
sync-version VERSION:
    cargo set-version -p const-str '{{VERSION}}'
