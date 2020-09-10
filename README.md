# const-str

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][docs-badge]][docs-url]
![CI][ci-badge]

[crates-badge]: https://img.shields.io/crates/v/const-str.svg
[crates-url]: https://crates.io/crates/const-str
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[docs-badge]: https://docs.rs/const-str/badge.svg
[docs-url]: https://docs.rs/const-str/
[ci-badge]: https://github.com/Nugine/const-str/workflows/CI/badge.svg

compile-time string operations

## Examples

```rust
assert_eq!(const_str::to_lowercase!("HELLO"), "hello");

assert_eq!(const_str::to_uppercase!("hello"), "HELLO");

assert_eq!(const_str::replace!("this is old", "old", "new"), "this is new");
```
